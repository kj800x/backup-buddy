use std::{sync::Arc, time::Duration};

use crate::{config, db::dao::PolicyDao};

pub struct Worker {
    __policy_dao: Arc<PolicyDao>,
    __config: Arc<config::Config>,
}

impl Worker {
    pub fn new(policy_dao: Arc<PolicyDao>, config: Arc<config::Config>) -> Self {
        Self {
            __policy_dao: policy_dao,
            __config: config,
        }
    }

    pub async fn run(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
