use dioxus::prelude::*;
use shield_bootstrap::BootstrapDioxusStyle;

pub fn client(app: fn() -> Element) {
    LaunchBuilder::new()
        .with_context(BootstrapDioxusStyle::default().context())
        .launch(app)
}
