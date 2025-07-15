use crate::{db::models::BackupPolicy, pages::layout};
use chrono::{Duration, TimeDelta};
use maud::{html, Markup};

pub fn index(policies: Vec<BackupPolicy>) -> Markup {
    html! {
        div.container
            hx-get="/backup-buddy/index-fragment"
            hx-trigger="load, every 1s"
            hx-swap="morph:outerHTML"
        {
            h1 { "Backup Policies" }

            div class="actions" {
                a href="/backup-buddy/policy/new" class="btn btn-primary" {
                    "Create New Policy"
                }
            }

            @if policies.is_empty() {
                p class="empty-message" { "No backup policies found" }
            } else {
                table class="groups-table" {
                    thead {
                        tr {
                            th { "Path" }
                            th { "Max Staleness" }
                            th { "Kind" }
                            th { "Recursive" }
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
                                    td { (layout::duration_element(Duration::milliseconds(policy.max_staleness as i64))) }
                                    td { (policy.kind.to_string()) }
                                    td { (if policy.recursive { "Yes" } else { "No" }) }
                                }
                            }
                    }
                }
            }
        }
    }
}

pub fn create_policy_form() -> Markup {
    html! {
        div class="form-container" {
            h1 { "Create New Backup Policy" }

            form
                hx-post="/backup-buddy/policy/create"
                hx-target="body"
                hx-swap="outerHTML"
                class="policy-form"
            {
                div class="form-group" {
                    label for="path" { "Path:" }
                    input
                        type="text"
                        id="path"
                        name="path"
                        required
                        placeholder="/path/to/backup"
                    {}
                }

                div class="form-group" {
                    label for="max_staleness" { "Max Staleness (milliseconds):" }
                    input
                        type="number"
                        id="max_staleness"
                        name="max_staleness"
                        required
                        min="0"
                        value="86400000"
                    {}
                }

                div class="form-group" {
                    label for="kind" { "Policy Kind:" }
                    select id="kind" name="kind" required {
                        option value="backup" { "Backup" }
                        option value="exclude" { "Exclude" }
                        option value="null" { "Null" }
                    }
                }

                div class="form-group" {
                    label class="checkbox-label" {
                        input type="checkbox" id="recursive" name="recursive" checked {}
                        span { "Recursive" }
                    }
                }

                div class="form-actions" {
                    button type="submit" class="btn btn-primary" { "Create Policy" }
                    a href="/backup-buddy" class="btn btn-secondary" { "Cancel" }
                }
            }
        }
    }
}
