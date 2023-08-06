use leptos::*;

use crate::pages::home;
use home::Home;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Home />
    }
}
