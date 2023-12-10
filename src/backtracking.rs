pub struct BacktrackNode {
    // An instance of a try at backtracking. Every node should be stored in a
    // list, where the order matters. The first node is the first try at backtracking
    // we made. We then add more nodes if the first bet wasn't enough to solve the
    // board.

    // Row index of the cell
    cell_row_idx: usize,
    // Column index of the cell
    cell_col_idx: usize,
    // All possible numbers for this cell
    possible_numbers: Vec<u8>,
    // The state of the board before backtracking
    initial_board: [[u8; 9]; 9],
    // The state of the blacklist before backtracking
    initial_blacklist: [[[u8; 9]; 9]; 9],
}

impl BacktrackNode {
    pub fn pop_next(&mut self) -> u8 {
        // Returns the next num from possible_numbers Vector. Returns 0 if Vector is empty.
        // if self.idx > 8 {
        //     return 0;
        // }
        // let curr: u8 = self.possible_numbers[self.idx];
        // self.idx += 1;
        // if curr == 0 {
        //     return self.pop_next();
        // } else {
        //     return curr;
        // }
        match self.possible_numbers.pop() {
            None => 0,
            Some(i) => i,
        }
    }

    pub fn new(cell_row_idx: usize, cell_col_idx: usize, possible_numbers: Vec<u8>, initial_board: [[u8; 9]; 9], initial_blacklist: [[[u8; 9]; 9]; 9]) -> Self {
        // Creates a new Backtrack Node
        Self {
            cell_row_idx,
            cell_col_idx,
            possible_numbers,
            initial_board,
            initial_blacklist,
        }
    }

    pub fn row_idx(&self) -> usize {
        self.cell_row_idx
    }

    pub fn col_idx(&self) -> usize {
        self.cell_col_idx
    }

    pub fn board(&self) -> [[u8; 9]; 9] {
        self.initial_board
    }

    pub fn blacklist(&self) -> [[[u8; 9]; 9]; 9] {
        self.initial_blacklist
    }
}
