use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
        <section class="text-center bg-red-400">
            <h1 class="">"Hello from home"</h1>
            <p class="">"asdf asdf"</p>
        </section>
    }
}
