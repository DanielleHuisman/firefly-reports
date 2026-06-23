mod app;
#[cfg(not(feature = "server"))]
mod client;
mod components;
#[cfg(feature = "server")]
mod server;
mod views;

use crate::app::App;

#[cfg(not(feature = "server"))]
fn main() {
    use crate::client::client;

    client(App)
}

#[cfg(feature = "server")]
fn main() {
    use crate::server::server;

    dioxus::serve(|| async move { server(App).await });
}
