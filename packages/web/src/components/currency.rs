use std::str::FromStr;

use dioxus::prelude::*;
use icu::{decimal::DecimalFormatter, locale::locale};
use rust_decimal::Decimal;

thread_local! {
    static FORMATTER: DecimalFormatter =
        #[expect(clippy::expect_used)]
        DecimalFormatter::try_new(locale!("nl").into(), Default::default())
            .expect("locale should be present");
}

#[component]
pub fn Currency(amount: Decimal) -> Element {
    // TODO: Replace `rust_decimal` with `fixed_decimal` to avoid conversion.
    let amount = FORMATTER.with(|formatter| {
        icu::decimal::input::Decimal::from_str(&amount.to_string())
            .map(|amount| formatter.format_to_string(&amount))
            .unwrap_or_else(|_| "NaN".to_owned())
    });

    rsx! {
        span {
            class: "font-monospace",
            "€ {amount}"
        }
    }
}
