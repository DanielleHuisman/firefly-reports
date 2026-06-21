use dioxus::prelude::*;
use firefly_reports_api::{ProfitAndLossReport, ReportGroup};
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
                        colspan: 3,
                        "Debit"
                    }
                    th {
                        class: "border-top border-bottom border-start border-end",
                        colspan: 3,
                        "Credit"
                    }
                }
                tr {
                    th {
                        class: "border-top border-bottom border-start",
                        "Category"
                    }
                    th {
                        class: "border-top border-bottom",
                        "Amount"
                    }
                    th {
                        class: "border-top border-bottom border-end",
                        "Total"
                    }
                    th {
                        class: "border-top border-bottom border-start",
                        "Category"
                    }
                    th {
                        class: "border-top border-bottom",
                        "Amount"
                    }
                    th {
                        class: "border-top border-bottom border-end",
                        "Total"
                    }
                }
            }
            tbody {
                for group in report.groups {
                    Group {
                        group
                    }
                }

                tr {
                    th {
                        class: "border-top border-bottom border-start",
                        colspan: 2,
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
                        colspan: 2,
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

#[component]
fn Group(group: ReportGroup) -> Element {
    rsx! {
        tr {
            td {
                class: "border-start border-end fw-bold",
                colspan: 3,
                "{group.name}"
            }
            td {
                class: "border-start border-end fw-bold",
                colspan: 3,
                "{group.name}"
            }
        }

        {group.debit
            .into_iter()
            .zip_longest(group.credit)
            .map(|pair| {
                match pair {
                    EitherOrBoth::Both(left, right) => {
                        rsx! {
                            tr {
                                td {
                                    class:"border-start",
                                    "{left.name}"
                                }
                                td {
                                    class: "text-end",
                                    Currency {
                                        amount: left.amount
                                    }
                                }
                                td {
                                    class: "border-end"
                                }
                                td {
                                    class: "border-start",
                                    "{right.name}"
                                }
                                td {
                                    class: "text-end",
                                    Currency {
                                        amount: right.amount
                                    }
                                }
                                td {
                                    class: "border-end"
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
                                    class: "text-end",
                                    Currency {
                                        amount: left.amount
                                    }
                                }
                                td {
                                    class: "border-end",
                                }
                                td {
                                    class: "border-start border-end",
                                    colspan: 3
                                }
                            }
                        }
                    },
                    EitherOrBoth::Right(right) => {
                        rsx! {
                            tr {
                                td {
                                    class: "border-start border-end",
                                    colspan: 3
                                },
                                td {
                                    class: "border-start",
                                    "{right.name}"
                                },
                                td {
                                    class: "text-end",
                                    Currency {
                                        amount: right.amount
                                    }
                                }
                                td {
                                    class: "border-end"
                                }
                            }
                        }
                    }
                }
            })
        }

        tr {
            td {
                class: "border-start",
                colspan: 2,
            }
            td {
                class: "border-end text-end fw-bold",
                Currency {
                    amount: group.debit_total,
                }
            }
            td {
                class: "border-start",
                colspan: 2,
            }
            td {
                class: "border-end text-end fw-bold",
                Currency {
                    amount: group.credit_total,
                }
            }
        }
    }
}
