pub struct TestResult {
    pub word_count: usize,
    pub time_taken: f64,
    pub wpm: f64,
}

pub fn calculate_wpm(word_count: usize, time_taken: f64) -> f64 {
    if time_taken <= 0.0 {
        0.0
    } else {
        word_count as f64 * 60.0 / time_taken
    }
}
