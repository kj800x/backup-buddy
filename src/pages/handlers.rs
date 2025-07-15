use actix_web::{web, HttpResponse, Responder};
use tracing::error;

use super::layout;
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
