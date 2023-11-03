use std::collections::HashSet;
use std::time::{Duration, Instant};

struct Sudoku {
    // Flat list of the sudoku board.
    board: [[u8; 9]; 9],
    // Create a blacklist of every number that can't be placed on its specific index.
    // This is a 2D Array where for eg. the first array is the list of numbers that can't be
    // placed in the first cell of the input.
    blacklist: [[u8; 9]; 81],
}

fn main() {
    let mut sudoku: Sudoku = Sudoku::new_easy();

    sudoku.draw();

    let correct: bool = sudoku.is_solved();
    println!("{}", correct.to_string());

    let start: Instant = Instant::now();
    sudoku.solve();
    let duration: Duration = start.elapsed();
    println!("Time elapsed to solve sudoku: {:?}", duration);

    sudoku.draw();

    let correct: bool = sudoku.is_solved();
    println!("{}", correct.to_string());
}

impl Sudoku {
    fn solve(&mut self) {
        // Solve the Sudoku (the crux!)
        self.blacklist[0][2] = 12;
        //dbg!(self.blacklist);

        while !self.is_solved() {
            // 1. Perform several checks to add entries to the blacklist
            // for cell in board
            // vertical lines check
            // horizontal lines check

            // 2. Fill cells of the board where the blacklist is only missing one number
            break;
        }
    }

    fn new_easy() -> Self {
        // TEMPORARY: Returns an easy to solve Sudoku
        Self {
            board: [
                [0, 3, 4, 6, 7, 8, 9, 1, 2], // 5
                [6, 7, 2, 1, 9, 5, 3, 4, 8],
                [1, 9, 8, 3, 4, 2, 5, 6, 7],
                [8, 5, 9, 7, 6, 1, 4, 2, 3],
                [4, 2, 6, 8, 5, 3, 7, 9, 1],
                [7, 1, 3, 9, 2, 4, 0, 5, 6], // 8
                [9, 6, 1, 5, 3, 7, 2, 8, 4],
                [2, 8, 7, 4, 1, 9, 6, 3, 5],
                [3, 4, 5, 2, 8, 0, 1, 7, 9], // 6
            ],
            blacklist: [[0u8; 9]; 81],
        }
    }

    fn new_hard() -> Self {
        // TEMPORARY: Returns a hard to solve Sudoku
        Self {
            board: [
                [8, 5, 0, 0, 0, 2, 4, 0, 0],
                [7, 2, 0, 0, 0, 0, 0, 0, 9],
                [0, 0, 4, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 7, 0, 0, 2, 3],
                [0, 5, 0, 0, 0, 9, 0, 0, 0],
                [4, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 8, 0],
                [0, 0, 7, 0, 0, 1, 7, 0, 0],
                [0, 0, 0, 3, 6, 0, 4, 0, 0],
            ],
            blacklist: [[0u8; 9]; 81],
        }
    }

    fn draw(&self) {
        // Print the Sudoku in a readable manner
        println!("+-----------------------------+");
        for i in 0..81 {
            let val: u8 = self.board[i / 9][i % 9];
            let cell_separator_left: &str = if i % 9 == 0 { "|" } else { "" };
            let cell_value: String = if val == 0 { String::from(" ") } else { val.to_string() };
            let cell_separator_right: &str = if i % 3 == 2 { "|" } else { "" };
            let line_end: &str = if i % 9 == 8 { "\n" } else { "" };
            
            print!("{} {} {}{}", cell_separator_left, cell_value, cell_separator_right, line_end);
            
            if i % 27 == 26 && i < 80 {
                println!("|---------+---------+---------|");
            }// else if i % 9 == 8 && i < 80 {
            //     println!("|         |         |         |");
            // }
        }
        println!("+-----------------------------+");
    }
    
    fn is_solved(&self) -> bool {
        // Return true if the sudoku is solved, false if not
        // Empty cells
        for row in self.board {
            if row.contains(&0) {
                return false;
            }
        }

        // Test rows
        for row in self.board {
            if has_duplicates(&row) {
                return false;
            }
        }

        // Test columns
        for i in 0..9 { // Column index
            let mut column: [u8; 9] = [0; 9];
            for j in 0..9 { // Vertical index
                column[j] = self.board[j][i];
            }

            if has_duplicates(&column) {
                return false;
            }
        }

        // Test 3x3
        // for i in 0..3 { // Box horizontal index
        //     for j in 0..3 { // Box vertical index
        //         let mut boxi: [u8; 9] = [0; 9];
        //         for box_x in 0..3 {
        //             for box_y in 0..3 {
        //                 boxi[box_x * 3 + box_y] = board[(i + box_x) * 9 + (j + box_y)];
        //             }
        //         }
        //         dbg!(boxi);
        
        //         if has_duplicates(&boxi) {
        //             return false;
        //         }
        //     }
        // }
        true
    }

    fn load(path: String) -> Self {
        // Load a sudoku from a file and return a Sudoku struct
        Self {
            board: [[0u8; 9]; 9],
            blacklist: [[0u8; 9]; 81],
        }
    }
}

fn has_duplicates(arr: &[u8]) -> bool {
    // TODO Move to a util file
    let mut set: HashSet<u8> = HashSet::new();

    for &element in arr {
        if set.contains(&element) {
            return true; // Found a duplicate
        }
        set.insert(element);
    }

    false // No duplicates found
}
