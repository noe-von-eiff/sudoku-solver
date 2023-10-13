use std::collections::HashSet;

fn main() {
    let input: [u8; 81] = [
        8, 5, 0, 0, 0, 2, 4, 0, 0,
        7, 2, 0, 0, 0, 0, 0, 0, 9,
        0, 0, 4, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 7, 0, 0, 2, 3,
        0, 5, 0, 0, 0, 9, 0, 0, 0,
        4, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 8, 0,
        0, 0, 7, 0, 0, 1, 7, 0, 0,
        0, 0, 0, 3, 6, 0, 4, 0, 0,
    ];
    let input: [u8; 81] = [
        5, 3, 4, 6, 7, 8, 9, 1, 2,
        6, 7, 2, 1, 9, 5, 3, 4, 8,
        1, 9, 8, 3, 4, 2, 5, 6, 7,
        8, 5, 9, 7, 6, 1, 4, 2, 3,
        4, 2, 6, 8, 5, 3, 7, 9, 1,
        7, 1, 3, 9, 2, 4, 8, 5, 6,
        9, 6, 1, 5, 3, 7, 2, 8, 4,
        2, 8, 7, 4, 1, 9, 6, 3, 5,
        3, 4, 5, 2, 8, 6, 1, 7, 9,
    ];

    solve(&input);
}

fn solve(board: &[u8; 81]) {
    // Create a blacklist of every number that can't be placed on its specific index.
    // This is a 2D Array where the first array is the list of numbers that can't be
    // placed in the first cell of the input.
    let mut blacklist = [[0u8; 9]; 81];
    blacklist[0][2] = 12;
    //dbg!(blacklist);
    draw_sudoku(&board);
    let correct = is_solved(&board);
    println!("{}", correct.to_string());
}

fn draw_sudoku(board: &[u8; 81]) {
    println!("+-----------------------------+");
    for (i, &val) in board.iter().enumerate() {
        let cell_separator_left = if i % 9 == 0 { "|" } else { "" };
        let cell_value = if val == 0 { String::from(" ") } else { val.to_string() };
        let cell_separator_right = if i % 3 == 2 { "|" } else { "" };
        let line_end = if i % 9 == 8 { "\n" } else { "" };
        
        print!("{} {} {}{}", cell_separator_left, cell_value, cell_separator_right, line_end);
        
        if i % 27 == 26 && i < 80 {
            println!("|---------+---------+---------|");
        }// else if i % 9 == 8 && i < 80 {
        //     println!("|         |         |         |");
        // }
    }
    println!("+-----------------------------+");
}

fn is_solved(board: &[u8; 81]) -> bool {
    // Empty cells
    if board.contains(&0) {
        return false;
    }

    // Test rows
    for i in 0..9 { // Row index
        let row: &[u8] = &board[9 * i .. 9 * (i + 1)];
        if has_duplicates(row) {
            return false;
        }
    }

    // Test columns
    for i in 0..9 { // Column index
        let mut column: [u8; 9] = [0; 9];
        for j in 0..9 { // Vertical index
            column[j] = board[j * 9 + i];
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

fn has_duplicates(arr: &[u8]) -> bool {
    let mut set = HashSet::new();

    for &element in arr {
        if set.contains(&element) {
            return true; // Found a duplicate
        }
        set.insert(element);
    }

    false // No duplicates found
}
