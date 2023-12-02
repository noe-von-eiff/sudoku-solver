pub struct BacktrackNode {
    // An instance of a try at backtracking. Every node should be stored in a
    // list, where the order matters. The first node is the first try at backtracking
    // we made. We then add more nodes if the first bet wasn't enough to solve the
    // board.

    // Row index of the cell
    pub cell_row_idx: usize,
    // Column index of the cell
    pub cell_col_idx: usize,
    // All possible numbers for this cell
    pub possible_numbers: Vec<u8>,
    // The state of the board before backtracking
    pub initial_board: [[u8; 9]; 9],
    // The state of the blacklist before backtracking
    pub initial_blacklist: [[[u8; 9]; 9]; 9],
}

impl BacktrackNode {
    pub fn pop_next(&mut self) -> u8 {
        // Returns the next num from possible_numbers Vector. Returns 0 if Vector is empty.
        match self.possible_numbers.pop() {
            None => 0,
            Some(i) => i,
        }
    }
}
