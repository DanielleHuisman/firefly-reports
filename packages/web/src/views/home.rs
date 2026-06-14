use dioxus::prelude::*;
use firefly_reports_api::profit_and_loss_report;
use itertools::{EitherOrBoth, Itertools};

use crate::components::currency::Currency;

#[component]
pub fn Home() -> Element {
    let started_at = "2026-01-01";
    let ended_at = "2026-01-31";

    let report = use_loader(async || {
        profit_and_loss_report(started_at.to_owned(), ended_at.to_owned()).await
    })?;
    let report = report.read();

    rsx! {
        div {
            class: "container",

            h1 {
                "Profit and Loss"
            }

            p {
                "{started_at} - {ended_at}"
            }

            table {
                class: "table table-borderless table-sm",
                style: "table-layout: fixed;",

                thead {
                    tr {
                        th {
                            class: "border-top border-bottom border-start border-end",
                            colspan: 2,
                            "Debit"
                        }
                        th {
                            class: "border-top border-bottom border-start border-end",
                            colspan: 2,
                            "Credit"
                        }
                    }
                    tr {
                        th {
                            class: "border-top border-bottom border-start",
                            "Category"
                        }
                        th {
                            class: "border-top border-bottom border-end",
                            "Amount"
                        }
                        th {
                            class: "border-top border-bottom border-start",
                            "Category"
                        }
                        th {
                            class: "border-top border-bottom border-end",
                            "Amount"
                        }
                    }
                }
                tbody {
                    {report.debit
                        .iter()
                        .zip_longest(report.credit.iter())
                        .map(|pair| {
                            match pair {
                                EitherOrBoth::Both(left, right) => {
                                    rsx! {
                                        tr {
                                            td {
                                                class: "border-start",
                                                "{left.name}"
                                            }
                                            td {
                                                class: "border-end text-end",
                                                Currency {
                                                    amount: left.amount
                                                }
                                            }
                                            td {
                                                class: "border-start",
                                                "{right.name}"
                                            }
                                            td {
                                                class: "border-end text-end",
                                                Currency {
                                                    amount: right.amount
                                                }
                                            }
                                        }
                                    }
                                },
                                EitherOrBoth::Left(left) => {
                                    rsx! {
                                        tr {
                                            td {
                                                class: "border-start",
                                                "{left.name}"
                                            },
                                            td {
                                                class: "border-end text-end",
                                                Currency {
                                                    amount: left.amount
                                                }
                                            }
                                            td {
                                                class: "border-start border-end",
                                                colspan: 2
                                            }
                                        }
                                    }
                                },
                                EitherOrBoth::Right(right) => {
                                    rsx! {
                                        tr {
                                            td {
                                                class: "border-start border-end",
                                                colspan: 2
                                            },
                                            td {
                                                class: "border-end",
                                                "{right.name}"
                                            },
                                            td {
                                                class: "border-start text-end",
                                                Currency {
                                                    amount: right.amount
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        })
                    }

                    tr {
                        th {
                            class: "border-top border-bottom border-start",
                            "Total"
                        }
                        th {
                            class: "border-top border-bottom border-end text-end",
                            Currency {
                                amount: report.debit_total
                            }
                        }
                        th {
                            class: "border-top border-bottom border-start",
                            "Total"
                        }
                        th {
                            class: "border-top border-bottom border-end text-end",
                            Currency {
                                amount: report.credit_total
                            }
                        }
                    }
                }
            }
        }
    }
}
