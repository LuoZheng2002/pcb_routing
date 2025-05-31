use leptos::{prelude::*, tachys::view};
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[component]
pub fn App() -> impl IntoView {
    // Reactive signals for rows and columns
    let (rows, set_rows) = signal::<usize>(10);
    let (cols, set_cols) = signal::<usize>(10);
    fn create_new_grid(rows: usize, cols: usize)-> Vec<Vec<Color>>{
        let mut rng = rand::rng();
        (0..rows)
            .map(|_| {
                (0..cols)
                    .map(|_| Color {
                        r: rng.random_range(0..=255),
                        g: rng.random_range(0..=255),
                        b: rng.random_range(0..=255),
                    })
                    .collect()
            })
            .collect()
    }
    let (grid, set_grid) = signal::<Vec<Vec<Color>>>(create_new_grid(rows.get(), cols.get()));
    
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
                    .iter()
                    .map(|row| {
                        view! {
                            <div style="display: flex; flex-direction: row;">
                                {row.iter()
                                    .map(|Color { r, g, b }| {
                                        let color_str = format!("rgb({},{},{})", r, g, b);
                                        view! {
                                            <div style=format!(
                                                "width: 40px; height: 40px; background-color: {}; display: inline",
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
    }
}
