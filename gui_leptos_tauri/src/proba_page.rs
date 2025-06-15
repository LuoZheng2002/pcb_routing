use leptos::{prelude::*, tachys::view, task::spawn_local};
use serde_wasm_bindgen::{from_value, to_value};
use shared::interface_types::{ClickCellArgs, Color, ColorGrid, MyResult, NewGridArgs, UpdatePosteriorArgs};
use wasm_bindgen::prelude::*;

use crate::app::invoke;


#[component]
pub fn ProbaPage() -> impl IntoView {
    let (err_msg, set_err_msg) = signal::<String>(String::new());
    // Reactive signals for rows and columns
    let (rows, set_rows) = signal::<usize>(25);
    let (cols, set_cols) = signal::<usize>(25);
    
    let (r, set_r) = signal::<u8>(0);
    let (g, set_g) = signal::<u8>(0);
    let (b, set_b) = signal::<u8>(0);

    let (score_weight, set_score_weight) = signal::<f64>(0.5);
    let (opportunity_cost_weight, set_opportunity_cost_weight) = signal::<f64>(0.5);
    fn create_new_grid(rows: usize, cols: usize)-> ColorGrid{
        let color_grid = (0..rows)
            .map(|_| {
                (0..cols)
                    .map(|_| Color {
                        r: 255,
                        g: 255,
                        b: 255,
                })
                .collect()
            })
            .collect();
        ColorGrid{grid: color_grid}
    }
    let (grid, set_grid) = signal::<ColorGrid>(create_new_grid(rows.get(), cols.get()));
    
    let proba_clear = move |e| {
        spawn_local(async move{
            set_err_msg.set("fetching grid...".to_string());
            let args = NewGridArgs {
                rows: rows.get(),
                cols: cols.get(),
            };
            let args = to_value(&args).unwrap();
            let result = invoke("proba_clear", args).await;
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

    let on_cell_click = move |x: usize, y: usize| {
        // You can replace this with a Signal or any effect/handler you prefer
        set_err_msg.set(format!(
            "Clicked cell at ({}, {}) with color rgb({}, {}, {})",
            x, y, r.get(), g.get(), b.get()
        ));
        spawn_local(async move {
            set_err_msg.set("clicking cell...".to_string());
            let args = to_value(&ClickCellArgs {
                x,
                y,
                r: r.get(),
                g: g.get(),
                b: b.get(),
            })
            .unwrap();
            let result = invoke("proba_click_cell", args).await;
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

    let on_color_click = move |color: Color|{
        set_r.set(color.r);
        set_g.set(color.g);
        set_b.set(color.b);
    };
    let on_init_click = move |_| {
        spawn_local(async move{
            set_err_msg.set("initializing iteration 1".to_string());
            let result = invoke("proba_init", JsValue::NULL).await;
            set_err_msg.set("iteration 1 initialized".to_string());
            let result = from_value::<MyResult<ColorGrid, String>>(result).unwrap();
            match result {
                MyResult::Ok(grid) => {
                    set_grid.set(grid);
                    set_err_msg.set("Iteration 1 completed".to_string());
                }
                MyResult::Err(err) => {
                    set_err_msg.set(err);
                }
            }
        });
    };
    let on_update_posterior_click = move |_| {
        spawn_local(async move{
            set_err_msg.set("updating posterior".to_string());
            let args = UpdatePosteriorArgs {
                scoreWeight: score_weight.get(),
                opportunityCostWeight: opportunity_cost_weight.get(),
            };
            let args = to_value(&args).unwrap();
            let result = invoke("proba_update_posterior", args).await;
            set_err_msg.set("posterior updated".to_string());
            let result = from_value::<MyResult<ColorGrid, String>>(result).unwrap();
            match result {
                MyResult::Ok(grid) => {
                    set_grid.set(grid);
                    set_err_msg.set("Posterior update completed".to_string());
                }
                MyResult::Err(err) => {
                    set_err_msg.set(err);
                }
            }
        });
    };
    let on_next_net_click = move |_|{
        spawn_local(async move{   
            set_err_msg.set("sampling next net".to_string());
            let result = invoke("proba_next_net", JsValue::NULL).await;
            set_err_msg.set("next net sampled".to_string());
            let result = from_value::<MyResult<ColorGrid, String>>(result).unwrap();
            match result {
                MyResult::Ok(grid) => {
                    set_grid.set(grid);
                    set_err_msg.set("Next net sampling completed".to_string());
                }
                MyResult::Err(err) => {
                    set_err_msg.set(err);
                }
            }
        });
    };
    let on_next_pair_click = move |_|{
        spawn_local(async move{
            set_err_msg.set("sampling next pair".to_string());
            let result = invoke("proba_next_pair", JsValue::NULL).await;
            set_err_msg.set("next pair sampled".to_string());
            let result = from_value::<MyResult<ColorGrid, String>>(result).unwrap();
            match result {
                MyResult::Ok(grid) => {
                    set_grid.set(grid);
                    set_err_msg.set("Next pair sampling completed".to_string());
                }
                MyResult::Err(err) => {
                    set_err_msg.set(err);
                }
            }
        });
    };
    let on_sample_click= move |_| {
        spawn_local(async move{
            set_err_msg.set("sampling".to_string());
            let result = invoke("proba_sample", JsValue::NULL).await;
            set_err_msg.set("sampled".to_string());
            let result = from_value::<MyResult<ColorGrid, String>>(result).unwrap();
            match result {
                MyResult::Ok(grid) => {
                    set_grid.set(grid);
                    set_err_msg.set("Next pair sampling completed".to_string());
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
            // rows, cols, new_grid, r, g, b, naive route, Bayesian route
                <label>"Rows: "</label>
                <input
                style="width: 3rem;"
                    type="number"
                    min="1"
                    prop:value=rows
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                            set_rows.set(val);
                        }
                    }
                />
                <label style="margin-left: 1rem;">"Cols: "</label>
                <input
                style="width: 3rem;"
                    type="number"
                    min="1"
                    prop:value=cols
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                            set_cols.set(val);
                        }
                    }
                />
                <button style="width: 6rem;" on:click=proba_clear>"Clear"</button>
            </div>
            <div>
                <button style="width: 6rem;" on:click=move|_| on_color_click(Color{r: 255, g: 0, b: 0})>"Red"</button>
                <button style="width: 6rem;" on:click=move|_| on_color_click(Color{r: 0, g: 255, b: 0})>"Green"</button>
                <button style="width: 6rem;" on:click=move|_| on_color_click(Color{r: 0, g: 0, b: 255})>"Blue"</button>
                <button style="width: 6rem;" on:click=move|_| on_color_click(Color{r: 255, g: 255, b: 0})>"Yellow"</button>
                <button style="width: 6rem;" on:click=move|_| on_color_click(Color{r: 255, g: 0, b: 255})>"Magenta"</button>
                <button style="width: 6rem;" on:click=move|_| on_color_click(Color{r: 0, g: 255, b: 255})>"Cyan"</button>
                <span style:background-color=move||{format!("#{:06x}", ((r.get() as u32) << 16) + ((g.get() as u32) << 8) + b.get() as u32)}>"Color"</span>
            </div>
            <div>
                <button style="width: 6rem;" on:click=on_init_click>"Init"</button>
                <button style="width: 8rem;" on:click=on_update_posterior_click>"Update Posterior"</button>
                <button style="width: 8rem;" on:click=on_sample_click>"Sample New Traces"</button>
            </div>
            <div>
                <button style="width: 6rem;" on:click=on_next_net_click>"Next Net"</button>
                <button style="width: 6rem;" on:click=on_next_pair_click>"Next Pair"</button>
                
            </div>
            <div>                
                <input
                    id="slider"
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value=move || score_weight.get().to_string()
                    on:input=move |ev| {
                        // Parse the slider value from the input event
                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                            set_score_weight.set(val);
                            if score_weight.get() + opportunity_cost_weight.get() > 1.0 {
                                set_opportunity_cost_weight.set(1.0 - score_weight.get());
                            } 
                        }
                    }
                />
                <label for="slider">"Score weight: "{move || score_weight.get().to_string()}</label>
                <input
                    id="slider"
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value=move || opportunity_cost_weight.get().to_string()
                    on:input=move |ev| {
                        // Parse the slider value from the input event
                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                            set_opportunity_cost_weight.set(val);
                            if score_weight.get() + opportunity_cost_weight.get() > 1.0 {
                                set_score_weight.set(1.0 - opportunity_cost_weight.get());
                            }
                        }
                    }
                />
                <label for="slider">"Opptortunity cost weight: "{move || opportunity_cost_weight.get().to_string()}</label>
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
                    .grid
                    .iter()
                    .enumerate()
                    .map(|(row_idx, row)| {
                        view! {
                            <div style="display: flex; flex-direction: row;">
                                {row.iter()
                                    .enumerate()
                                    .map(|(col_idx, Color{r, g, b})| {
                                        let color_str = format!("rgb({},{},{})", r, g, b);
                                        let y = row_idx;
                                        let x = col_idx;
                                        view! {
                                            <div style=format!(
                                                "width: 16px; height: 16px; background-color: {}; border: 2px solid black; display: inline",
                                                color_str,
                                            )
                                            on:click=move |_| on_cell_click(x, y)
                                            ></div>
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