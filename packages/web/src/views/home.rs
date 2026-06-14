use dioxus::prelude::*;
use firefly_reports_api::{profit_and_loss_report, user};

#[component]
pub fn Home() -> Element {
    let user = use_loader(user)?;
    let user = user.read();

    let started_at = "2026-01-01";
    let ended_at = "2026-01-31";

    let report = use_loader(async || {
        profit_and_loss_report(started_at.to_owned(), ended_at.to_owned()).await
    })?;
    let _report = report.read();

    rsx! {
        for email_address in &user.email_addresses {
            span { "{email_address}" }
        }
    }
}
