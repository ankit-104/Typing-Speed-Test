use crate::storage;
use std::io;

use super::modes::self_writing;

pub fn run() {
    let mut best_score = storage::load_best_score();
    loop {
        println!("------------------------------");
        println!("      Typing Speed Test       ");
        println!("------------------------------");
        println!("Menu:");
        println!("1 -> Self writing test");
        println!("2 -> View best score");
        println!("3 -> Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the choice");
        let mut choice = choice.trim();
        match choice {
            "1" => {
                println!("Start typing...");
                let result = self_writing::start();
                if (result.wpm > best_score) {
                    best_score = result.wpm;
                    storage::save_best_score(best_score);
                }
                println!("No of words typed : {}", result.word_count);
                println!("Time taken : {}", result.time_taken);
                println!("WPM : {}", result.wpm);
                println!("Best Score {best_score}");
            }
            "2" => println!("Best Score : {best_score}"),
            "3" => break,
            _ => println!("Please enter a valid input!"),
        }
        println!();
    }
}
