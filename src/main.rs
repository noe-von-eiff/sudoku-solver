use crate::sudoku::Sudoku;
use std::time::Instant;

mod utils;
mod sudoku;
mod backtracking;

fn main() {
    let mut sudoku: Sudoku = Sudoku::load("boards/inkala.sdku");

    sudoku.draw();
    println!("{}", sudoku.is_solved());

    let start: Instant = Instant::now();
    sudoku.solve();
    println!("Time elapsed to solve sudoku: {:?}", start.elapsed());

    sudoku.draw();

    println!("{}", sudoku.is_solved());
}
