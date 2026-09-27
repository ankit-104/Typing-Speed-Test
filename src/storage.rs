use std::fs;

pub fn load_best_score() -> f64 {
    let mut best_score = fs::read_to_string("best_score.txt");
    match best_score {
        Ok(val) => val.trim().parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    }
}

pub fn save_best_score(score: f64) {
    fs::write("best_score.txt", score.to_string()).expect("Unable to update best score");
}
