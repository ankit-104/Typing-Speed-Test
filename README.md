# Typing Speed Test

A simple Rust command‑line app that measures how many words you can type.

## Build & Run
```bash
cargo run --release   # starts the interactive menu
```

- **Self‑writing test** – type a passage; the program reports word count and elapsed time.
- **Best score** – saved in `best_score.txt` (ignored by git).

## Project layout
- `src/app.rs` – UI and menu.
- `src/modes/self_writing.rs` – timing and word‑count logic.
- `src/storage/best_score.rs` – load/display the best score.
- Other modules (`stats`, `test`, `input`) are placeholders for future work.

## License
MIT – feel free to fork or improve.
