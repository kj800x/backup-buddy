use actix_web::{web, HttpResponse, Responder};
use std::str::FromStr;
use tracing::error;
use uuid::Uuid;

use super::layout;
use crate::db::models::{BackupPolicy, PolicyKind};
use crate::AppState;

pub async fn web_index(state: web::Data<AppState>) -> impl Responder {
    let policy_dao = state.policy_dao.clone();
    let policies = match web::block(move || policy_dao.get_policies()).await {
        Ok(Ok(policies)) => policies,
        Ok(Err(e)) => {
            error!(error = %e, "Failed to list policies");
            Vec::new()
        }
        Err(e) => {
            error!(error = ?e, "Blocking error while listing policies");
            Vec::new()
        }
    };
    let content = super::web::index(policies);
    HttpResponse::Ok().body(layout::base_layout("Backup Buddy", content).into_string())
}

pub async fn web_index_fragment(state: web::Data<AppState>) -> impl Responder {
    let policy_dao = state.policy_dao.clone();
    let policies = match web::block(move || policy_dao.get_policies()).await {
        Ok(Ok(policies)) => policies,
        Ok(Err(e)) => {
            error!(error = %e, "Failed to list policies");
            Vec::new()
        }
        Err(e) => {
            error!(error = ?e, "Blocking error while listing policies");
            Vec::new()
        }
    };
    let content = super::web::index(policies);
    HttpResponse::Ok().body(content.into_string())
}

pub async fn create_policy_form() -> impl Responder {
    let content = super::web::create_policy_form();
    HttpResponse::Ok()
        .body(layout::base_layout("Create Policy - Backup Buddy", content).into_string())
}

pub async fn create_policy(
    state: web::Data<AppState>,
    form: web::Form<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let form_data = form.into_inner();

    // Parse form data
    let path = match form_data.get("path") {
        Some(p) if !p.trim().is_empty() => p.trim().to_string(),
        _ => {
            error!("Missing or empty path in form submission");
            return HttpResponse::BadRequest().body("Path is required");
        }
    };

    let max_staleness = match form_data
        .get("max_staleness")
        .and_then(|s| s.parse::<u64>().ok())
    {
        Some(ms) => ms,
        _ => {
            error!("Invalid max_staleness in form submission");
            return HttpResponse::BadRequest().body("Invalid max staleness value");
        }
    };

    let kind = match form_data
        .get("kind")
        .and_then(|k| PolicyKind::from_str(k).ok())
    {
        Some(k) => k,
        _ => {
            error!("Invalid policy kind in form submission");
            return HttpResponse::BadRequest().body("Invalid policy kind");
        }
    };

    let recursive = form_data.get("recursive").is_some();

    // Create new policy
    let policy = BackupPolicy {
        id: Uuid::new_v4(),
        path,
        max_staleness,
        kind,
        recursive,
    };

    // Save to database
    let policy_dao = state.policy_dao.clone();
    let policy_clone = policy.clone();
    match web::block(move || policy_dao.create_policy(&policy_clone)).await {
        Ok(Ok(())) => {
            // Redirect to index page
            HttpResponse::Found()
                .append_header(("Location", "/backup-buddy"))
                .body("")
        }
        Ok(Err(e)) => {
            error!(error = %e, "Failed to create policy");
            HttpResponse::InternalServerError().body("Failed to create policy")
        }
        Err(e) => {
            error!(error = ?e, "Blocking error while creating policy");
            HttpResponse::InternalServerError().body("Failed to create policy")
        }
    }
}

pub async fn policy_details(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let policy_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid policy ID");
        }
    };

    let policy_dao = state.policy_dao.clone();
    match web::block(move || policy_dao.get_policy(&policy_id)).await {
        Ok(Ok(Some(policy))) => {
            let content = super::web::policy_details(policy);
            HttpResponse::Ok()
                .body(layout::base_layout("Policy Details - Backup Buddy", content).into_string())
        }
        Ok(Ok(None)) => HttpResponse::NotFound().body("Policy not found"),
        Ok(Err(e)) => {
            error!(error = %e, "Failed to get policy");
            HttpResponse::InternalServerError().body("Failed to get policy")
        }
        Err(e) => {
            error!(error = ?e, "Blocking error while getting policy");
            HttpResponse::InternalServerError().body("Failed to get policy")
        }
    }
}
