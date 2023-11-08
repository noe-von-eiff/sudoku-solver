use crate::sudoku::Sudoku;
use std::time::Instant;

mod utils;
mod sudoku;

fn main() {
    let mut sudoku: Sudoku = Sudoku::load("boards/evil1.sdku");

    sudoku.draw();

    println!("{}", sudoku.is_solved().to_string());

    let start: Instant = Instant::now();
    sudoku.solve();
    println!("Time elapsed to solve sudoku: {:?}", start.elapsed());

    sudoku.draw();

    println!("{}", sudoku.is_solved().to_string());
}
