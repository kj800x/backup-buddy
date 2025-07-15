use chrono::{DateTime, Duration, Local, Utc};
use maud::{html, Markup};

pub fn __timestamp_element(timestamp: Option<DateTime<Utc>>) -> Markup {
    match timestamp {
        None => html! {
            span { "N/A" }
        },
        Some(utc_time) => {
            let local_time = utc_time.with_timezone(&Local);
            let now = Utc::now();
            let diff = now - utc_time;

            let relative_time = if diff < Duration::minutes(1) {
                "just now".to_string()
            } else if diff < Duration::hours(1) {
                let minutes = diff.num_minutes();
                format!(
                    "{} minute{} ago",
                    minutes,
                    if minutes == 1 { "" } else { "s" }
                )
            } else if diff < Duration::days(1) {
                let hours = diff.num_hours();
                format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
            } else if diff < Duration::days(7) {
                let days = diff.num_days();
                format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
            } else {
                let days = diff.num_days();
                format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
            };

            let formatted_time = local_time.format("%d %b %Y %I:%M:%S %p %Z").to_string();
            let iso_time = utc_time.to_rfc3339();

            html! {
                time datetime=(iso_time) title=(iso_time) {
                    (format!("{} ({})", relative_time, formatted_time))
                }
            }
        }
    }
}

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                link rel="stylesheet" href="/backup-buddy/assets/styles.css" {}
                (scripts())
            }
            body hx-ext="morph" {
                (content)
            }
        }
    }
}

/// Common scripts for all pages
pub fn scripts() -> Markup {
    html! {
        script src="/backup-buddy/assets/htmx.min.js" {}
        script src="/backup-buddy/assets/idiomorph.min.js" {}
        script src="/backup-buddy/assets/idiomorph-ext.min.js" {}
    }
}
