mod app;
mod naive_page;
mod home_page;
mod proba_page;
mod redirect_button;
mod nav_bar;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
