use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::{sync::Arc, time::Duration};
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;

mod data;

use crate::db::models::{BackupPolicy, PolicyKind};
use crate::{config, db::dao::PolicyDao};
use chrono::Utc;
pub use data::MissingPolicyReport;
pub use data::MissingPolicyWorkerData;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
struct NormalizedPolicy {
    id: Uuid,
    kind: PolicyKind,
    recursive: bool,
    prefix: PathBuf,
}

fn normalize_and_sort_policies(
    policies: Vec<BackupPolicy>,
    base_dir: &Path,
) -> Vec<NormalizedPolicy> {
    let mut result: Vec<_> = policies
        .into_iter()
        .map(|p| NormalizedPolicy {
            id: p.id,
            kind: p.kind,
            recursive: p.recursive,
            prefix: base_dir.join(p.path.trim_start_matches('/')),
        })
        .collect();

    result.sort_by_key(|p| std::cmp::Reverse(p.prefix.components().count()));
    result
}

fn find_most_specific_policy<'a>(
    path: &Path,
    policies: &'a [NormalizedPolicy],
) -> Option<&'a NormalizedPolicy> {
    policies.iter().find(|p| {
        if p.recursive {
            path.starts_with(&p.prefix)
        } else {
            path == p.prefix
        }
    })
}

/// Returns all policies that could potentially apply to the path or its
/// subdirectories. If this returns more than one policy, then we need to keep
/// recursing, otherwise we know that only the one policy can apply.
fn potential_relevant_policies<'a>(
    path: &Path,
    policies: &'a [NormalizedPolicy],
) -> Vec<&'a NormalizedPolicy> {
    policies
        .iter()
        .filter(|p| path.starts_with(&p.prefix))
        .collect()
}

pub struct MissingPolicyWorker {
    policy_dao: Arc<PolicyDao>,
    config: Arc<config::Config>,
    data: Arc<RwLock<MissingPolicyWorkerData>>,
}

impl MissingPolicyWorker {
    pub fn new(
        policy_dao: Arc<PolicyDao>,
        config: Arc<config::Config>,
        data: Arc<RwLock<MissingPolicyWorkerData>>,
    ) -> Self {
        Self {
            policy_dao,
            config,
            data,
        }
    }

    pub async fn build_report(
        &self,
        policies: Vec<BackupPolicy>,
        base_dir: PathBuf,
    ) -> MissingPolicyReport {
        let mut report = MissingPolicyReport {
            missing_paths: vec![],
            policy_hits: HashMap::new(),
        };

        let policies = normalize_and_sort_policies(policies, &base_dir);

        self.scan_dir(&base_dir, &policies, &mut report).await;

        report
    }

    /// FIXME: This seems to work so far, but it needs a ton of testing.
    fn scan_dir<'a>(
        &'a self,
        dir: &'a Path,
        policies: &'a [NormalizedPolicy],
        report: &'a mut MissingPolicyReport,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let mut read_dir = match fs::read_dir(dir).await {
                Ok(rd) => ReadDirStream::new(rd),
                Err(_) => return, // ignore unreadable dirs
            };

            while let Some(entry) = read_dir.next().await {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let path = entry.path();
                let file_type = match entry.file_type().await {
                    Ok(ft) => ft,
                    Err(_) => continue,
                };

                let policy = find_most_specific_policy(&path, policies);
                let potential_relevant_policies = potential_relevant_policies(&path, policies);

                match policy {
                    None
                    | Some(NormalizedPolicy {
                        kind: PolicyKind::Null,
                        ..
                    }) => {
                        if file_type.is_file() {
                            report.missing_paths.push(path);
                        } else if file_type.is_dir() {
                            if potential_relevant_policies.len() > 1 {
                                self.scan_dir(&path, policies, report).await;
                            } else {
                                report.missing_paths.push(path);
                            }
                        }
                    }
                    Some(NormalizedPolicy {
                        kind: PolicyKind::Exclude,
                        ..
                    }) => {
                        // skip
                    }
                    Some(NormalizedPolicy {
                        kind: PolicyKind::Backup,
                        id,
                        recursive,
                        ..
                    }) => {
                        if file_type.is_file() {
                            *report.policy_hits.entry(*id).or_insert(0) += 1;
                        } else if file_type.is_dir() {
                            // If there are polices that might apply to subdirs
                            // or the policy is not recursive we need to check
                            // subdirs.
                            if potential_relevant_policies.len() > 1 || !*recursive {
                                self.scan_dir(&path, policies, report).await;
                            }
                        }
                    }
                }
            }
        })
    }

    pub async fn run(&self) {
        loop {
            {
                let mut data = self.data.write().await;
                data.last_run_started = Some(Utc::now());
            }

            let policies = self
                .policy_dao
                .get_policies()
                .expect("Failed to get policies");

            let report = self
                .build_report(policies, self.config.base_dir.clone())
                .await;

            println!("{:?}", report);

            {
                let mut data = self.data.write().await;
                data.report = Some(report);
                data.last_run_completed = Some(Utc::now());
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}
