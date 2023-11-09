use crate::utils::has_duplicates;
use std::fs;

pub struct Sudoku {
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

impl Sudoku {
    pub fn solve(&mut self) {
        // Solve the Sudoku (the crux!)
        while !self.is_solved() {
            let mut is_changed: bool = false;

            // 1. Perform several checks to add entries to the blacklist
            'cell_iter:for i_cell in 0..81 { // Iterate over every cell
                let row_idx: usize = i_cell / 9;
                let col_idx: usize = i_cell % 9;

                // Skip filled cells
                if self.board[row_idx][col_idx] != 0 {
                    continue;
                }

                // Fill this cells blacklist with numbers that appear in the current row
                let row: [u8; 9] = self.board[row_idx];
                for num in row {
                    // If that rows cell is a number and the blacklist entry for this number in this cell is empty...
                    if num != 0 && self.blacklist[row_idx][col_idx][(num - 1) as usize] == 0 {
                        // ...add the num to the blacklist
                        self.blacklist[row_idx][col_idx][(num - 1) as usize] = num;
                        is_changed = true;
                    }
                }

                // Fill this cells blacklist with numbers that appear in the current column
                let mut column: [u8; 9] = [0; 9];
                for j in 0..9 { // Vertical index
                    column[j] = self.board[j][col_idx];
                }
                for num in column {
                    // If that columns cell is a number and the blacklist entry for this number in this cell is empty...
                    if num != 0 && self.blacklist[row_idx][col_idx][(num - 1) as usize] == 0 {
                        // ...add the num to the blacklist
                        self.blacklist[row_idx][col_idx][(num - 1) as usize] = num;
                        is_changed = true;
                    }
                }

                // Fill this cells blacklist with numbers that appear in the current 3x3 grid
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

                // Do some more complicated checks using the blacklist of a cells regions
                let cell_whitelist: Vec<u8> = self.blacklist[row_idx][col_idx]
                    .iter()
                    .enumerate()
                    .filter(|(_, &r)| r == 0)
                    .map(|(index, _)| (index + 1) as u8)
                    .collect(); // The numbers that could be put in this cell (basically a whitelist)

                for num in cell_whitelist {
                    let mut is_only_possible_num: bool = true; // True if this num is the only number that can be put in this cell

                    // Fill cell with x if all other empty cells in this row have x in their blacklist
                    // +-----------------------------+                 +-----------------------------+
                    // |         |    9  7 |       6 |                 |         |    9  7 |       6 |
                    // | 5       | 2       | 1     4 |                 | 5    *9*| 2       | 1     4 |
                    // | 3       |       1 |    7    |                 | 3       |       1 |    7    |
                    // |---------+---------+---------|                 |---------+---------+---------|
                    // |    9  3 | 8  2  5 |       7 |                 |    9  3 | 8  2  5 |       7 |
                    // |         | 9  1  4 |         | ----becomes---> |         | 9  1  4 |         |
                    // | 4       | 7  3  6 | 5  9    |                 | 4       | 7  3  6 | 5  9    |
                    // |---------+---------+---------|                 |---------+---------+---------|
                    // |    4    | 1       |       9 |                 |    4    | 1       |       9 |
                    // | 8     2 |       9 |       1 |                 | 8     2 |       9 |       1 |
                    // | 9       | 6  4    |         |                 | 9       | 6  4    |         |
                    // +-----------------------------+                 +-----------------------------+
                    for i in 0..9 { // Iterate through the row
                        if i != row_idx && self.board[i][col_idx] == 0 { // If we aren't on the current cell and the cell is empty
                            let temp_blacklist: [u8; 9] = self.blacklist[i][col_idx];
                            if !temp_blacklist.contains(&num) {
                                is_only_possible_num = false;
                                break;
                            }
                        }
                    }

                    if is_only_possible_num {
                        self.board[row_idx][col_idx] = num;
                        continue 'cell_iter; // Can directly go to the next cell since this one is now filled
                    }

                    // Fill cell with x if all other empty cells in this column have x in their blacklist
                    // +-----------------------------+                 +-----------------------------+
                    // | 1       |    9  7 |       6 |                 | 1       |    9  7 |       6 |
                    // | 5     9 | 2       | 1     4 |                 | 5 *7* 9 | 2       | 1     4 |
                    // | 3       |       1 | 9  7    |                 | 3       |       1 | 9  7    |
                    // |---------+---------+---------|                 |---------+---------+---------|
                    // | 6  9  3 | 8  2  5 | 4  1  7 |                 | 6  9  3 | 8  2  5 | 4  1  7 |
                    // | 2     7 | 9  1  4 |         | ----becomes---> | 2     7 | 9  1  4 |         |
                    // | 4       | 7  3  6 | 5  9    |                 | 4       | 7  3  6 | 5  9    |
                    // |---------+---------+---------|                 |---------+---------+---------|
                    // | 7  4    | 1       |       9 |                 | 7  4    | 1       |       9 |
                    // | 8     2 |    7  9 |    4  1 |                 | 8     2 |    7  9 |    4  1 |
                    // | 9       | 6  4    | 7       |                 | 9       | 6  4    | 7       |
                    // +-----------------------------+                 +-----------------------------+
                    is_only_possible_num = true; // Reset to true
                    for i in 0..9 { // Iterate through the column
                        if i != col_idx && self.board[row_idx][i] == 0 { // If we aren't on the current cell and the cell is empty
                            let temp_blacklist: [u8; 9] = self.blacklist[row_idx][i];
                            if !temp_blacklist.contains(&num) {
                                is_only_possible_num = false;
                                break;
                            }
                        }
                    }

                    if is_only_possible_num {
                        self.board[row_idx][col_idx] = num;
                        continue 'cell_iter; // Can directly go to the next cell since this one is now filled
                    }

                    // Fill cell with x if all other empty cells in this 3x3 grid have x in their blacklist
                    // +-----------------------------+                 +-----------------------------+
                    // |    4  5 | 2  8    | 7  9  6 |                 |    4  5 | 2  8    | 7  9  6 |
                    // |         |       4 | 1       |                 |         |       4 | 1       |
                    // |       9 |         | 4     3 |                 |       9 |         | 4     3 |
                    // |---------+---------+---------|                 |---------+---------+---------|
                    // | 9       | 7       | 5  6    |                 | 9       | 7       | 5  6    |
                    // |    8    | 5  1    | 9  4  7 | ----becomes---> |*6* 8    | 5  1    | 9  4  7 |
                    // | 7  5    |       9 |       1 |                 | 7  5    |       9 |       1 |
                    // |---------+---------+---------|                 |---------+---------+---------|
                    // | 4     6 |         | 2       |                 | 4     6 |         | 2       |
                    // | 5  2  7 | 4       |         |                 | 5  2  7 | 4       |         |
                    // |         |    2  5 | 6  7  4 |                 |         |    2  5 | 6  7  4 |
                    // +-----------------------------+                 +-----------------------------+
                    is_only_possible_num = true; // Reset to true
                    'grid_iter:for i in 0..3 { // Iterate through the 3x3 grid
                        for j in 0..3 {
                            let this_row_idx: usize = box_row_idx + i;
                            let this_col_idx: usize = box_col_idx + j;
                            // If we aren't on the current cell and the cell is empty
                            if !(this_row_idx == row_idx && this_col_idx == col_idx) && self.board[this_row_idx][this_col_idx] == 0 {
                                let temp_blacklist: [u8; 9] = self.blacklist[this_row_idx][this_col_idx];
                                if !temp_blacklist.contains(&num) {
                                    is_only_possible_num = false;
                                    break 'grid_iter;
                                }
                            }
                        }
                    }

                    if is_only_possible_num {
                        self.board[row_idx][col_idx] = num;
                        continue 'cell_iter; // Can directly go to the next cell since this one is now filled
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

    pub fn draw(&self) {
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
    
    pub fn is_solved(&self) -> bool {
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

    pub fn load(path: &str) -> Self {
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
