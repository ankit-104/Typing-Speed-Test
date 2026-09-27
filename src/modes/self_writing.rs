use crate::stats::{self, TestResult};
use std::time::Instant;

pub fn start() -> TestResult {
    // Calculate time and take input
    let start_time = Instant::now();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input....");
    let time_taken = start_time.elapsed().as_secs_f64();
    let word_count = input.split_whitespace().count();

    // calculate the wpm
    let wpm = stats::calculate_wpm(word_count, time_taken);

    TestResult {
        word_count,
        time_taken,
        wpm,
    }
}
