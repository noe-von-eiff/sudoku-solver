use std::collections::HashSet;
use std::time::Instant;
use std::fs;

struct Sudoku {
    // 2D 9x9 matrix of the sudoku board.
    board: [[u8; 9]; 9],
    // Create a blacklist of every number that can't be placed on a specific cell.
    // This is a 3D 9x9x9 matrix where the first 2 indices define the cell and the last
    // index returns 0 if the number could be placed on that cell or a number, if that number
    // can't be placed on that cell. Eg: blacklist[0][0] may return [1, 2, 3, 4, 0, 6, 7, 8, 9]
    // which would tell us that every number except for the number 5 cannot be placed in that
    // cell, thus the number 5 should be inserted in board[0][0].
    blacklist: [[[u8; 9]; 9]; 9],
}

fn main() {
    let mut sudoku: Sudoku = Sudoku::load("boards/easy1.sdku");

    sudoku.draw();

    println!("{}", sudoku.is_solved().to_string());

    let start: Instant = Instant::now();
    sudoku.solve();
    println!("Time elapsed to solve sudoku: {:?}", start.elapsed());

    sudoku.draw();

    println!("{}", sudoku.is_solved().to_string());
}

impl Sudoku {
    fn solve(&mut self) {
        // Solve the Sudoku (the crux!)
        while !self.is_solved() {
            let mut is_changed: bool = false;

            // 1. Perform several checks to add entries to the blacklist
            for i in 0..81 { // Iterate over every cell
                let row_idx: usize = i / 9;
                let col_idx: usize = i % 9;

                let val: u8 = self.board[row_idx][col_idx];

                // Skip filled cells
                if val != 0 {
                    continue;
                }

                // Vertical check
                let row: [u8; 9] = self.board[row_idx];
                for num in row {
                    // If that rows cell isn't empty and the blacklist entry for this number in this cell is empty...
                    if num != 0 && self.blacklist[row_idx][col_idx][(num - 1) as usize] == 0 {
                        // ...add the num to the blacklist
                        self.blacklist[row_idx][col_idx][(num - 1) as usize] = num;
                        is_changed = true;
                    }
                }

                // Horizontal check
                let mut column: [u8; 9] = [0; 9];
                for j in 0..9 { // Vertical index
                    column[j] = self.board[j][col_idx];
                }
                for num in column {
                    // If that columns cell isn't empty and the blacklist entry for this number in this cell is empty...
                    if num != 0 && self.blacklist[row_idx][col_idx][(num - 1) as usize] == 0 {
                        // ...add the num to the blacklist
                        self.blacklist[row_idx][col_idx][(num - 1) as usize] = num;
                        is_changed = true;
                    }
                }

                // 3x3 check
                let box_row_idx: usize = (row_idx / 3) * 3; // Row index of this 3x3 grids top left cell
                let box_col_idx: usize = (col_idx / 3) * 3; // Column index of this 3x3 grids top left cell
                let mut grid: [u8; 9] = [0u8; 9]; // Representation of the 3x3 grid this cell is in
                for i in 0..3 {
                    for j in 0..3 {
                        grid[j + 3 * i] = self.board[box_row_idx + i][box_col_idx + j];
                    }
                }
                for num in grid {
                    // If that 3x3 grids cell isn't empty and the blacklist entry for this number in this cell is empty...
                    if num != 0 && self.blacklist[row_idx][col_idx][(num - 1) as usize] == 0 {
                        // ..add the num to the blacklist
                        self.blacklist[row_idx][col_idx][(num - 1) as usize] = num;
                        is_changed = true;
                    }
                }
            }

            // 2. Fill cells of the board where the blacklist is only missing one number
            for i in 0..9 {
                for j in 0..9 {
                    // If we are talking about an empty cell...
                    if self.board[i][j] == 0 {
                        // ...check the blacklist for a possible number to insert
                        // If the amount of 0s in that cells blacklist is 1...
                        if self.blacklist[i][j].iter().filter(|&n| *n == 0).count() == 1 {
                            // ...the number at that index is the only possible entry for that cell
                            let index: usize = self.blacklist[i][j].iter().position(|&r| r == 0).unwrap();
                            self.board[i][j] = (index + 1) as u8;
                            is_changed = true;
                        }
                    }
                }
            }

            // 3. Check if a change occured
            if !is_changed {
                println!("Sudoku is not solvable with my current checks!");
                break;
            }
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
            }
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

        // Test 3x3 grid
        for grid_row_idx in 0..3 { // Iterrate over the grid indcies
            for grid_col_idx in 0..3 {
                let mut grid: [u8; 9] = [0u8; 9]; // Representation of the 3x3 grids content

                for i in 0..3 { // Horizontal index
                    for j in 0..3 { // Vertical index
                        // Insert cell of this grid at the correct index
                        grid[j + 3 * i] = self.board[grid_row_idx * 3 + i][grid_col_idx * 3 + j];
                    }
                }

                if has_duplicates(&grid) {
                    return false;
                }
            }
        }
        true
    }

    fn load(path: &str) -> Self {
        // Load a sudoku from a file and return a Sudoku struct
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");

        let contents: String = contents.replace("\n", "");
        let contents: String = contents.replace("\r", "");
        let contents: String = contents.replace(" ", "");

        let cell_numbers_as_str: Vec<&str> = contents.split(",").collect();
        let mut board: [[u8; 9]; 9] = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let as_str: &str = cell_numbers_as_str[j + 9 * i];
                let as_int: u8 = as_str.parse().expect("The sdku file contains non-numeric values!");
                board[i][j] = as_int;
            }
        }

        Self {
            board: board,
            blacklist: [[[0u8; 9]; 9]; 9],
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
