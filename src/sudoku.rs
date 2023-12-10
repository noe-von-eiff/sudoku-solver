use crate::backtracking::BacktrackNode;

pub struct Sudoku {
    // 2D 9x9 matrix of the sudoku board.
    board: [[u8; 9]; 9],
    // Blacklist of every number that can't be placed on a specific cell.
    blacklist: [[[u8; 9]; 9]; 9],
    // Stack of all the currently active backtracking nodes. If we can't go further with logic,
    // a new node is added to the stack.
    backtrack_stack: Vec<BacktrackNode>,
    // TODO maybe save the board in a hasmap instead of an array. maybe thats faster
    // maybe u could even have several hashmaps, one for columns, one for rows and one for boxes??
}

impl Sudoku {
    pub fn solve(&mut self) {
        // Solve the Sudoku (the crux!)
        let mut is_backtracking: bool = false; // Only true when other logic doesn't work
        while !self.is_solved() {
            let mut is_changed: bool = false;

            'cell_iter:for i_cell in 0..81 { // Iterate over every cell
                let row_idx: usize = i_cell / 9; // This calculation seems to be faster then 2 0..9 for-loops. idk why tho!
                let col_idx: usize = i_cell % 9;

                // Skip filled cells
                if self.board[row_idx][col_idx] != 0 {
                    continue;
                }

                // 1. Perform several checks to add entries to the blacklist

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
                let cell_whitelist: [u8; 9] = self.whitelist_for(row_idx, col_idx);
                for num in cell_whitelist {
                    if num == 0 {
                        continue;
                    }

                    let mut is_only_possible_num: bool = true; // True if this num is the only number that can be put in this cell

                    // Fill cell with x if all other empty cells in this row have x in their blacklist
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
                    is_only_possible_num = true; // Reset to true
                    for i in 0..9 { // Iterate through the column
                        // TODO This is wrong, self.board[row_idx][i] iterates through a row, not a column. Please make correct comments
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

                // 2. Fill cells of the board where the blacklist is only missing one number

                // If we are talking about an empty cell...
                if self.board[row_idx][col_idx] == 0 {
                    // ...check the blacklist for a possible number to insert
                    // If the amount of 0s in that cells blacklist is 1...
                    if self.blacklist[row_idx][col_idx].iter().filter(|&n| *n == 0).count() == 1 {
                        // ...the number at that index is the only possible entry for that cell
                        let index: usize = self.blacklist[row_idx][col_idx].iter().position(|&r| r == 0).unwrap();
                        self.board[row_idx][col_idx] = (index + 1) as u8;
                        is_changed = true;
                    }
                }

                // Handle Backtracking: Check if the backtracking has caused errors
                if is_backtracking {
                    // If this cell is empty, but its blacklist doesn't have any possible numbers left...
                    if self.board[row_idx][col_idx] == 0 && self.blacklist[row_idx][col_idx].iter().filter(|&n| *n == 0).count() == 0 {
                        // ... there is an error (caused by our bet) and we should backtrack!
                        self.reset_state(); // Reset to inital board and blacklist state
                        self.try_new_num(); // Try with the next possible number
                        is_changed = true;
                    }
                }
            }

            // 3. Check if a change occured
            if !is_changed {
                // Activate backtracking
                // println!("Backtracking!");
                is_backtracking = true;
                let (bb_row_idx, bb_col_idx) = self.compute_best_bet();
                // let cell_whitelist: [u8; 9] = self.whitelist_for(bb_row_idx, bb_col_idx);
                let cell_whitelist: Vec<u8> = self.blacklist[bb_row_idx][bb_col_idx]
                    .iter()
                    .enumerate()
                    .filter(|(_, &r)| r == 0)
                    .map(|(index, _)| (index + 1) as u8)
                    .collect();
                self.backtrack_stack.push(BacktrackNode::new(
                    bb_row_idx,
                    bb_col_idx,
                    cell_whitelist,
                    self.board.clone(),
                    self.blacklist.clone(),
                ));
                self.try_new_num(); // Try with the next possible number
            }
        }
    }

    fn try_new_num(&mut self) {
        // Inserts the next possible value on the board. Only use this method when backtracking!
        let mut node: BacktrackNode = self.backtrack_stack.pop().unwrap();
        let num_to_try: u8 = node.pop_next();

        if num_to_try == 0 { // No more nums in vector, means the num from parent backtrack node is wrong
            self.reset_state(); // Reset to inital board and blacklist state
            self.try_new_num(); // Try with the next possible number
        } else {
            self.board[node.row_idx()][node.col_idx()] = num_to_try;
            self.backtrack_stack.push(node); // Put the node back in our vector
        }
    }

    fn reset_state(&mut self) {
        // Resets the board and blacklist to the defined inital state. Only use this method when backtracking!
        let last_node: &BacktrackNode = self.backtrack_stack.last().unwrap();
        self.board = last_node.board().clone(); // Reset to initial board state
        self.blacklist = last_node.blacklist().clone(); // Reset to initial blacklist state
    }

    fn whitelist_for(&self, row_idx: usize, col_idx: usize) -> [u8; 9] {
        // Returns the numbers that could be put in this cell (basically a whitelist)
        let mut count: u8 = 0;
        self.blacklist[row_idx][col_idx]
            .map(|v: u8| {
                count += 1;
                if v == 0 {
                    count
                } else {
                    0
                }
            })
    }

    fn compute_best_bet(&self) -> (usize, usize) {
        // Returns the cell with the least amount of possible numbers that could be placed in it
        let mut best_bet_row_idx: usize = 0;
        let mut best_bet_col_idx: usize = 0;
        let mut best_bet_possible_numbers: usize = 9;
        // Iterate over every cell
        for row_idx in 0..9 { // Basically Y coordinate
            for col_idx in 0..9 { // Basically X coordinate
                // Skip filled cells
                if self.board[row_idx][col_idx] != 0 {
                    continue;
                }

                let possible_nums_count: usize = self.blacklist[row_idx][col_idx].iter().filter(|&n| *n == 0).count();
                if possible_nums_count < best_bet_possible_numbers {
                    best_bet_row_idx = row_idx;
                    best_bet_col_idx = col_idx;
                    best_bet_possible_numbers = possible_nums_count;

                    if best_bet_possible_numbers == 2 {
                        // Early return because 2 is the minimum we can possibly find
                        return (best_bet_row_idx, best_bet_col_idx);
                    }
                }
            }
        }
        (best_bet_row_idx, best_bet_col_idx)
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
        // Test rows
        for row in self.board {
            let sum: u8 = row.iter().sum();
            if sum != 45 {
                return false;
            }
        }

        // Test columns
        for i in 0..9 { // Column index
            let mut column: [u8; 9] = [0; 9];
            for j in 0..9 { // Vertical index
                column[j] = self.board[j][i];
            }

            let sum: u8 = column.iter().sum();
            if sum != 45 {
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

                let sum: u8 = grid.iter().sum();
                if sum != 45 {
                    return false;
                }
            }
        }
        true
    }

    pub fn from_string(str: &str) -> Self {
        // Turns a sudoku string into a Sudoku struct. A sudoku string should look like this: 0043002...
        let cell_numbers_as_str: Vec<u8> = str.split("")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut board: [[u8; 9]; 9] = [[0u8; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = *cell_numbers_as_str.get(j + 9 * i).unwrap();
            }
        }

        Self {
            board: board,
            blacklist: [[[0u8; 9]; 9]; 9],
            backtrack_stack: Vec::new(),
        }
    }
}
