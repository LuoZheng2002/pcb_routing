use std::{collections::HashMap, num::NonZeroUsize, sync::Mutex};

use lazy_static::lazy_static;

pub const LENGTH_PENALTY_RATE: f64 = 1.0;
pub const TURN_PENALTY_RATE: f64 = 3.0;
pub const HALF_PROBABILITY_RAW_SCORE: f64 = 10.0;

pub const MAX_TRACES_PER_ITERATION: usize = 4; // Maximum number of traces per iteration
pub const MAX_GENERATION_ATTEMPTS: usize = 10; // Maximum number of attempts to generate a trace

lazy_static! {
    pub static ref SCORE_WEIGHT: Mutex<f64> = Mutex::new(0.3);
    pub static ref OPPORTUNITY_COST_WEIGHT: Mutex<f64> = Mutex::new(0.3);
    pub static ref ITERATION_TO_PRIOR_PROBABILITY: HashMap<NonZeroUsize, f64> = {
        let mut map = HashMap::new();
        let mut remaining_probability = 1.0; // Start with a total probability of 1.0
        let probability_1 = 0.7; // Probability for the first iteration
        remaining_probability -= probability_1; // Subtract the first iteration's probability
        let probability_2 = remaining_probability * 0.7;
        remaining_probability -= probability_2; // Subtract the second iteration's probability
        let probability_3 = remaining_probability * 0.7; // Third iteration gets half of the remaining probability
        // remaining_probability -= probability_3; // Subtract the third iteration's probability
        map.insert(NonZeroUsize::new(1).unwrap(), probability_1);
        map.insert(NonZeroUsize::new(2).unwrap(), probability_2);
        map.insert(NonZeroUsize::new(3).unwrap(), probability_3);
        let mut probability_sum = 0.0;
        for value in map.values() {
            probability_sum += value;
            if probability_sum > 1.0 {
                panic!("Total prior probability exceeds 1.0: {}", probability_sum);
            }
        }
        map
    };
}
