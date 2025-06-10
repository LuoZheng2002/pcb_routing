use leptos::prelude::*;
use leptos_router::{components::A};



#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div>
            <A href="/">HomePage</A>
            <A href="/naive">NaivePage</A>
            <A href="/proba">ProbaPage</A>
        </div>
    }
}