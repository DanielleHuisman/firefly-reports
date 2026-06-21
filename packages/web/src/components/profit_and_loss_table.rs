use dioxus::prelude::*;
use firefly_reports_api::ProfitAndLossReport;
use itertools::{EitherOrBoth, Itertools};

use crate::components::currency::Currency;

#[component]
pub fn ProfitAndLossTable(report: ProfitAndLossReport) -> Element {
    rsx! {
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
