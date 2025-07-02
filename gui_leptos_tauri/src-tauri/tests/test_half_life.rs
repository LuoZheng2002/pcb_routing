use gui_leptos_tauri_lib::hyperparameters::HALF_PROBABILITY_RAW_SCORE;

#[test]
pub fn test_half_life() {
    let score_raw = 20.0;
    let k = f64::ln(2.0) / HALF_PROBABILITY_RAW_SCORE;
    let score = f64::exp(-k * score_raw);
}
