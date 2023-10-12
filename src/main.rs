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

    draw_sudoku(&input);
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
