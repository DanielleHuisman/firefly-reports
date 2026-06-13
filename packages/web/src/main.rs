mod app;
mod views;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
