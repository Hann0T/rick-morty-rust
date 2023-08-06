pub mod app;
pub mod pages;

use leptos::*;
use app::App;

fn main() {
    mount_to_body(|cx| view! {cx, <App />});
}
