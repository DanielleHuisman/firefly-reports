use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use shield_dioxus_axum::{ExtractShield, UserRequired};
#[cfg(feature = "server")]
use shield_memory::User as ShieldUser;

#[derive(Deserialize, PartialEq, Serialize)]
pub struct ProfitAndLossReport {}

#[get("/api/profit-and-loss-report?started_at&ended_at", shield: ExtractShield<ShieldUser>, user: UserRequired<ShieldUser>)]
pub async fn profit_and_loss_report(
    started_at: String,
    ended_at: String,
) -> Result<ProfitAndLossReport, ServerFnError> {
    use anyhow::Context;
    use firefly_iii::apis::{
        Api,
        categories_api::ListCategoryParams,
        insight_api::{InsightExpenseCategoryParams, InsightIncomeCategoryParams},
    };

    use crate::client::firefly_client;

    let client = firefly_client(shield, user).await?;

    let categories = client
        .categories_api()
        .list_category(ListCategoryParams::builder().limit(100).page(1).build())
        .await
        .context("failed to list categories")?;

    info!("{categories:#?}");

    let income_entries = client
        .insight_api()
        .insight_income_category(
            InsightIncomeCategoryParams::builder()
                .start(started_at.to_owned())
                .end(ended_at.to_owned())
                .build(),
        )
        .await
        .context("failed to list income by category")?;

    let expense_entries = client
        .insight_api()
        .insight_expense_category(
            InsightExpenseCategoryParams::builder()
                .start(started_at.to_owned())
                .end(ended_at.to_owned())
                .build(),
        )
        .await
        .context("failed to list expenses by category")?;

    info!("{income_entries:#?}");
    info!("{expense_entries:#?}");

    Ok(ProfitAndLossReport {})
}
