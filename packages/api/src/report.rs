use dioxus::prelude::*;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use shield_dioxus_axum::{ExtractShield, UserRequired};
#[cfg(feature = "server")]
use shield_memory::User as ShieldUser;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProfitAndLossReport {
    pub credit: Vec<ReportCategory>,
    pub credit_total: Decimal,
    pub debit: Vec<ReportCategory>,
    pub debit_total: Decimal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ReportCategory {
    pub id: String,
    pub name: String,
    pub amount: Decimal,
}

#[get("/api/profit-and-loss-report?start&end", shield: ExtractShield<ShieldUser>, user: UserRequired<ShieldUser>)]
pub async fn profit_and_loss_report(
    start: String,
    end: String,
) -> Result<ProfitAndLossReport, ServerFnError> {
    use anyhow::Context;
    use firefly_iii::apis::{
        Api,
        categories_api::ListCategoryParams,
        insight_api::{InsightExpenseCategoryParams, InsightIncomeCategoryParams},
    };
    use futures::join;
    use itertools::Itertools;
    use rust_decimal::dec;

    use crate::client::firefly_client;

    let client = firefly_client(shield, user).await?;

    let (categories, income_entries, expense_entries) = join!(
        client
            .categories_api()
            .list_category(ListCategoryParams::builder().limit(100).page(1).build()),
        client.insight_api().insight_income_category(
            InsightIncomeCategoryParams::builder()
                .start(start.to_owned())
                .end(end.to_owned())
                .build(),
        ),
        client.insight_api().insight_expense_category(
            InsightExpenseCategoryParams::builder()
                .start(start.to_owned())
                .end(end.to_owned())
                .build(),
        )
    );

    let categories = categories.context("failed to list categories")?;
    let income_entries = income_entries.context("failed to list income by category")?;
    let expense_entries = expense_entries.context("failed to list expenses by category")?;

    let entries = income_entries
        .into_iter()
        .chain(expense_entries)
        .sorted_by(|a, b| a.id.cmp(&b.id))
        .chunk_by(|entry| entry.id.clone());

    let categories: Vec<ReportCategory> = entries
        .into_iter()
        .filter_map(|(category_id, entries)| {
            category_id.and_then(|category_id| {
                categories
                    .data
                    .iter()
                    .find(|category| category.id == category_id)
                    .map(|category| {
                        entries
                            .into_iter()
                            .try_fold(dec!(0), |acc, entry| {
                                // TODO: Support multiple currencies.

                                entry
                                    .difference
                                    .map(|difference| Decimal::from_str_exact(&difference))
                                    .unwrap_or(Ok(dec!(0)))
                                    .map(|difference| acc + difference)
                            })
                            .map(|amount| ReportCategory {
                                id: category.id.clone(),
                                name: category.attributes.name.clone(),
                                amount,
                            })
                    })
            })
        })
        .try_collect()
        .context("failed to process entries")?;

    let (mut credit, mut debit): (Vec<_>, Vec<_>) = categories
        .into_iter()
        .partition(|category| category.amount >= dec!(0));

    for entry in &mut credit {
        entry.amount = entry.amount.round_dp(2);
    }
    for entry in &mut debit {
        entry.amount = (-entry.amount).round_dp(2);
    }

    credit.sort_by(|a, b| a.name.cmp(&b.name));
    debit.sort_by(|a, b| a.name.cmp(&b.name));

    let credit_total = credit
        .iter()
        .map(|category| category.amount)
        .sum::<Decimal>()
        .round_dp(2);
    let debit_total = debit
        .iter()
        .map(|category| category.amount)
        .sum::<Decimal>()
        .round_dp(2);

    Ok(ProfitAndLossReport {
        credit,
        credit_total,
        debit,
        debit_total,
    })
}
