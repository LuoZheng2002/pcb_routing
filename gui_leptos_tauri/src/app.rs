use leptos::prelude::*;
use leptos_router::{components::{ParentRoute, Route, Router, Routes}, path};
use wasm_bindgen::prelude::*;

use crate::{home_page::HomePage, naive_page::NaivePage, nav_bar::NavBar, proba_page::ProbaPage};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


// pub async fn terminal_log(message: &str) {
//     invoke(
//         "log",
//         to_value(&LogArgs {
//             message: message.to_string(),
//         })
//         .unwrap(),
//     )
//     .await;
// }





#[component]
pub fn App() -> impl IntoView {
    view! {
    <div>Hello world</div>
    <div id="root">
      // we wrap the whole app in a <Router/> to allow client-side navigation
      // from our nav links below
      <NavBar/>
      <Router>
        <main>
          // <Routes/> both defines our routes and shows them on the page
          <Routes fallback=|| "Not found.">
              // users like /gbj or /bob
              <Route
                path=path!("/")
                view=HomePage
              />
              <Route
                path=path!("/naive")
                view=NaivePage
              />
              <Route
                path=path!("/proba")
                view=ProbaPage
              />
              // a fallback if the /:id segment is missing from the URL
              <Route
                path=path!("")
                view=move || view! { <p class="contact">"Select a contact."</p> }
              />
          </Routes>
        </main>
      </Router>
    </div>
  }
}
