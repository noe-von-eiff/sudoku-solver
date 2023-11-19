use crate::sudoku::Sudoku;
use std::time::{Instant, Duration};

mod utils;
mod sudoku;

fn main() {
    let mut sudoku: Sudoku = Sudoku::load("boards/easy3.sdku");

    sudoku.draw();
    println!("{}", sudoku.is_solved());

    let start: Instant = Instant::now();
    sudoku.solve();
    println!("Time elapsed to solve sudoku: {:?}", start.elapsed());

    sudoku.draw();

    println!("{}", sudoku.is_solved());
    // avg_time();
}

fn avg_time() {
    // TODO mabye make this part of the Sudoku impl or maybe as a Test? The input arg could the be some path to a board
    const N: f64 = 25.0;
    let mut total_duration: Duration = Duration::new(0, 0);
    for _ in 0..(N as i32) {
        let mut sudoku: Sudoku = Sudoku::load("boards/challenge.sdku");
        let start: Instant = Instant::now();
        sudoku.solve();
        total_duration += start.elapsed();
    }
    println!("Solving took an average of {:.4}ms", (total_duration.as_secs_f64() / N) * 1000.0);
}
