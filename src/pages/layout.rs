use chrono::{DateTime, Duration, Local, TimeDelta, Utc};
use maud::{html, Markup};

fn human_readable_duration(duration: TimeDelta) -> String {
    let mut remainder = duration;
    let mut components: Vec<String> = Vec::new();

    if remainder >= Duration::days(1) {
        components.push(format!("{}d", remainder.num_days()));
        remainder = remainder - Duration::days(remainder.num_days());
    }
    if remainder >= Duration::hours(1) {
        components.push(format!("{}h", remainder.num_hours()));
        remainder = remainder - Duration::hours(remainder.num_hours());
    }
    if remainder >= Duration::minutes(1) {
        components.push(format!("{}m", remainder.num_minutes()));
        remainder = remainder - Duration::minutes(remainder.num_minutes());
    }
    if remainder >= Duration::seconds(1) {
        components.push(format!("{}s", remainder.num_seconds()));
        remainder = remainder - Duration::seconds(remainder.num_seconds());
    }
    if remainder >= Duration::milliseconds(1) {
        components.push(format!("{}ms", remainder.num_milliseconds()));
    }

    components.join(" ")
}

pub fn duration_element(duration: TimeDelta) -> Markup {
    html! {
        time datetime=(duration.to_string()) title=(format!("{} ms", duration.num_milliseconds())) { (human_readable_duration(duration)) }
    }
}

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
