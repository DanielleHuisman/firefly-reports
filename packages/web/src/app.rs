use dioxus::prelude::*;
use shield_dioxus::ShieldRouter;

use crate::views::{Home, ProfitAndLoss};

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/profit-and-loss?:start&:end")]
        ProfitAndLoss {
            start: Option<String>,
            end: Option<String>,
        },
    #[child("/auth")]
    Auth {
        child: ShieldRouter
    },
}

#[component]
pub fn App() -> Element {
    rsx! {
        Stylesheet {
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css",
            integrity: "sha384-sRIl4kxILFvY47J16cr9ZwB07vP4J8+LH7qKQnuqkuIAvNWLzeN8tE5YBujZqJLB",
            crossorigin: "anonymous"
        }

        document::Link { rel: "icon", href: FAVICON }

        Title { "Firefly III Reports" }

        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |error_context: ErrorContext| {
                let error_string = format!("{:?}", error_context.error());
                let is_unauthorized = error_string.contains("401") || error_string.contains("Unauthorized");

                let route = use_route::<Route>();

                rsx! {
                    div {
                        class: "container",

                        if is_unauthorized {
                            div {
                                h1 { "Unauthorized" }
                                p { "You must be signed in to view this page." }
                                a {
                                    class: "btn btn-primary",
                                    href: "/auth/sign-in?redirectUrl={route}",
                                    "Sign in"
                                }
                            }
                        } else {
                            div {
                                h1 { "Error" }
                                p { "An unexpected error occurred." }
                                p { "{error_string}" }
                                button { onclick: move |_| error_context.clear_errors(), "Try again" }
                            }
                        }
                    }
                }
            },

            Outlet::<Route> {}
        }
    }
}
