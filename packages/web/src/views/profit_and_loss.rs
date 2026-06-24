use dioxus::{core::throw_error, prelude::*};
use firefly_reports_api::profit_and_loss_report;
use jiff::{ToSpan, Zoned, civil::Date};

use crate::components::profit_and_loss_table::ProfitAndLossTable;

#[derive(Clone)]
struct Dates {
    start: Date,
    end: Date,
}

#[component]
pub fn ProfitAndLoss(start: Option<String>, end: Option<String>) -> Element {
    let dates = use_signal(|| {
        #[expect(clippy::expect_used)]
        Dates {
            start: start
                .and_then(|start| start.parse().ok())
                .unwrap_or_else(|| Zoned::now().first_of_month().expect("valid date").date()),
            end: end
                .and_then(|end| end.parse().ok())
                .unwrap_or_else(|| Zoned::now().last_of_month().expect("valid date").date()),
        }
    });

    let report = use_resource(move || async move {
        let dates = dates();
        profit_and_loss_report(dates.start.to_string(), dates.end.to_string()).await
    });

    rsx! {
        div {
            class: "container",

            h1 {
                "Profit and Loss"
            }

            match &*report.read_unchecked() {
                Some(Ok(report)) => rsx! {
                    Navigation {
                        dates
                    }

                    ProfitAndLossTable {
                        report: report.clone()
                    }

                    Navigation {
                        dates
                    }
                },
                Some(Err(err)) => {
                    throw_error(err.clone());
                    rsx! {}
                },
                None => rsx! {
                    div {
                        class: "spinner-border",
                        role: "status",

                        span {
                            class: "visually-hidden",

                            "Loading..."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Navigation(dates: Signal<Dates>) -> Element {
    rsx! {
        div {
            class: "d-flex flex-wrap align-items-center justify-content-between gap-2 mb-2",

            div {
                class: "d-flex flex-wrap align-items-center gap-2",

                button {
                    class: "btn btn-primary",
                    onclick: move |_| dates.with_mut(|dates| {
                        dates.start -= 1.year();
                        dates.end -= 1.year();
                    }),
                    "«"
                }

                button {
                    class: "btn btn-primary",
                    onclick: move |_| dates.with_mut(|dates| {
                        dates.start -= 1.month();
                        dates.end -= 1.month();
                    }),
                    "‹"
                }

                div {
                    input {
                        class: "form-control",
                        name: "start",
                        placeholder: "Start",
                        "aria-label": "Start",
                        type: "date",
                        value: "{dates().start}",
                        onchange: move |event| {
                            if let Ok(date) = event.value().parse() {
                                dates.with_mut(|dates| dates.start = date);
                            }
                            Ok(())
                        }
                    }
                }

                div {
                    input {
                        class: "form-control",
                        name: "end",
                        placeholder: "End",
                        "aria-label": "End",
                        type: "date",
                        value: "{dates().end}",
                        onchange: move |event| {
                            if let Ok(date) = event.value().parse() {
                                dates.with_mut(|dates| dates.end = date);
                            }
                            Ok(())
                        }
                    }
                }

                button {
                    class: "btn btn-primary",
                    onclick: move |_| dates.with_mut(|dates| {
                        dates.start += 1.month();
                        dates.end += 1.month();
                    }),
                    "›"
                }

                button {
                    class: "btn btn-primary",
                    onclick: move |_| dates.with_mut(|dates| {
                        dates.start += 1.year();
                        dates.end += 1.year();
                    }),
                    "»"
                }
            }

            div {
                class: "d-flex flex-wrap align-items-center gap-2",

                button {
                    class: "btn btn-primary flex-grow-1 flex-md-grow-0",
                    style: "min-width: 10rem;",
                    onclick: move |_| dates.with_mut(|dates| {
                        #[expect(clippy::expect_used)]
                        {
                            dates.start = Zoned::now().first_of_month().expect("valid date").date();
                            dates.end = Zoned::now().last_of_month().expect("valid date").date();
                        }
                    }),
                    "Current month"
                }
                button {
                    class: "btn btn-primary flex-grow-1 flex-md-grow-0",
                    style: "min-width: 10rem;",
                    onclick: move |_| dates.with_mut(|dates| {
                        #[expect(clippy::expect_used)]
                        {
                            dates.start = Zoned::now().first_of_year().expect("valid date").date();
                            dates.end = Zoned::now().last_of_year().expect("valid date").date();
                        }
                    }),
                    "Current year"
                }
            }
        }
    }
}
