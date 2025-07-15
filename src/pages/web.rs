use crate::db::models::BackupPolicy;
use maud::{html, Markup};

pub fn index(policies: Vec<BackupPolicy>) -> Markup {
    html! {
        div.container
            hx-get="/backup-buddy/index-fragment"
            hx-trigger="load, every 1s"
            hx-swap="morph:outerHTML"
        {
            h1 { "Backup Policies" }

            @if policies.is_empty() {
                p class="empty-message" { "No backup policies found" }
            } else {
                table class="groups-table" {
                    thead {
                        tr {
                            th { "Path" }
                            th { "Max Staleness" }
                        }
                    }
                    tbody {
                            @for policy in policies {
                                tr {
                                    td {
                                        a href=(format!("/backup-buddy/policy/{}", policy.id)) class="link" {
                                            (policy.path)
                                        }
                                    }
                                    td { (policy.max_staleness) }
                                }
                            }
                    }
                }
            }
        }
    }
}
