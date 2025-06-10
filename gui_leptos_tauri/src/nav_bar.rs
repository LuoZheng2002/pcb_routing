use leptos::prelude::*;
use leptos_router::{components::A};



#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <A href="/">" Home Page "</A>
            <A href="/naive">" Naive Page "</A>
            <A href="/proba">" Proba Page "</A>
        </nav>
    }
}