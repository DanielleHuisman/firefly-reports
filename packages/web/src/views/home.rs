use dioxus::prelude::*;
use firefly_reports_api::user;

#[component]
pub fn Home() -> Element {
    let user = use_loader(user)?;
    let user = user.read();
    let user = user
        .name
        .as_ref()
        .unwrap_or(&user.email_addresses[0])
        .as_str();

    rsx! {
        div {
            class: "container",

            h1 {
                "Reports"
            }

            p {
                "{user}"
            }

            ul {
                li {
                    a {
                        href: "/profit-and-loss",
                        "Profit and Loss"
                    }
                }
            }
        }
    }
}
