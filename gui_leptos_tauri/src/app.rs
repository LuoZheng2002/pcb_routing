use leptos::{prelude::*, tachys::view, task::spawn_local};
use rand::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use shared::datatypes::{Color, ColorGrid, Grid, MyResult};
use wasm_bindgen::prelude::*;

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
    let (err_msg, set_err_msg) = signal::<String>(String::new());
    // Reactive signals for rows and columns
    let (rows, set_rows) = signal::<usize>(10);
    let (cols, set_cols) = signal::<usize>(10);
    fn create_new_grid(rows: usize, cols: usize)-> ColorGrid{
        let mut rng = rand::rng();
        let color_grid = (0..rows)
            .map(|_| {
                (0..cols)
                    .map(|_| Color (
                        rng.random_range(0..=255),
                        rng.random_range(0..=255),
                        rng.random_range(0..=255),
                    ))
                    .collect()
            })
            .collect();
        ColorGrid(color_grid)
    }
    let (grid, set_grid) = signal::<ColorGrid>(create_new_grid(rows.get(), cols.get()));
    
    let fetch_grid = move || {
        spawn_local(async move{
            set_err_msg.set("fetching grid...".to_string());
            let result = invoke("fetch_grid", JsValue::NULL).await;
            set_err_msg.set("grid fetched".to_string());
            let result = from_value::<MyResult<ColorGrid, String>>(result).unwrap();
            match result {
                MyResult::Ok(grid) => {                    
                    set_grid.set(grid);
                }
                MyResult::Err(err) => {
                    set_err_msg.set(err);
                }
            }
        });        
    };


    view! {
        <div style="padding: 1rem;">
            <div style="margin-bottom: 1rem;">
                <label>"Rows: "</label>
                <input
                    type="number"
                    min="1"
                    prop:value=rows
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                            set_rows.set(val);
                            let grid = 
                            set_grid.set(create_new_grid(val, cols.get()));
                        }
                    }
                />
                <label style="margin-left: 1rem;">"Cols: "</label>
                <input
                    type="number"
                    min="1"
                    prop:value=cols
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                            set_cols.set(val);
                            set_grid.set(create_new_grid(rows.get(), val));
                        }
                    }
                />
                <button on:click=move |_| fetch_grid()>"Fetch Grid"</button>
            </div>

            <div style="
            width: 600px;
            height: 400px;
            overflow: scroll;
            border: 1px solid #ccc;
            ">
            <div style="width: fit-content; height: fit-content;">
                {move ||{grid
                    .get()
                    .0
                    .iter()
                    .map(|row| {
                        view! {
                            <div style="display: flex; flex-direction: row;">
                                {row.iter()
                                    .map(|Color(r, g, b)| {
                                        let color_str = format!("rgb({},{},{})", r, g, b);
                                        view! {
                                            <div style=format!(
                                                "width: 40px; height: 40px; background-color: {}; border: 2px solid black; display: inline",
                                                color_str,
                                            )></div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        }                        
                    })
                    .collect::<Vec<_>>()}
                }
                </div>
            </div>
        </div>
        <div style="color: rgb(255, 0, 0)">{err_msg}</div>
    }
}
