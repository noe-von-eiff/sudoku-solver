# Sudoku Solver
## Motivation
I love sudoku! I like to solve puzzles on the Pink Pointer Sudoku App when I'm bored. Seeing how hard
the Hard and Evil puzzles are, I was wondering if a basic sudoku solver using human 
strategies would be able to solve these hard puzzles. This also proved to be 
a prefect first relatively big project I can make using Rust, as I am currently learning the language!

## Goals and Accomplishments
The initial goal of this project was to make a sudoku solver that uses human strategies to
solve the hardest puzzles in the Pink Pointer Sudoku App. The App has four difficulty stages:
Easy, Medium, Hard and Evil. I copied the first 3 puzzles from each category for testing.

I also looked into how fast other solvers are online and stumbled upon 
[this small challenge on StackExchange](https://codegolf.stackexchange.com/questions/190727/the-fastest-sudoku-solver).
The goal of this challenge is to use an algorithm to solve a given list of 49k 17-clue boards. The official winner of 
the challenge used Tdoku and got a solving time of 201ms for all the 49k boards! This solver takes a whole lot longer:
240 seconds are needed to solve all 49k boards. There is a lot of work ahead if I want to make this faster!

The hardest challenge I've set for myself is to make the solver be able to solve the "World's most difficult
Sudoku" designed by Arto Inkala in 2012. The puzzle is featured and talked about
[here](https://abcnews.go.com/blogs/headlines/2012/06/can-you-solve-the-hardest-ever-sudoku) and 
[here](https://sudoku2.com/play-the-hardest-sudoku-in-the-world/). This proved quite challenging because the logic
strategies I implemented simply aren't enough, so I had to add a last resort backtracking algorithm. If logic isn't
enough, the solver takes a bet on a cell and tries different possible numbers. With the backtracking algorithm
implemented, the solver solves the Inkala board in about 50ms!

## Usage
Here I'll explain the few methods that exist and what they are used for.
### Example
This short example shows how to load a board, draw it in the console and solve it.
```rust
use crate::sudoku::Sudoku;
use std::time::Instant;

mod utils;
mod sudoku;

fn main() {
    let mut sudoku: Sudoku = Sudoku::from_string("004080300000003042800405907302070508050000070608090201406207009520900000007010400"); // Load a sudoku board from a string

    sudoku.draw(); // Draw the current state of the board on the console

    println!("{}", sudoku.is_solved()); // Print wether the board is solved or not

    let start: Instant = Instant::now(); // Measure time it takes to solve
    sudoku.solve(); // Solve the board
    println!("Time elapsed to solve sudoku: {:?}", start.elapsed()); // Print time it took to solve

    sudoku.draw(); // Draw the solved board

    println!("{}", sudoku.is_solved()); // Should print 'true' now!
}
```
### `from_string()`
Returns a `Sudoku` struct. Requires a string representation of a board. The string must only
contain digits and be 81 characters long. An empty cell is represented by a 0. Each digit
will be inserted in the board, left to right, top to bottom. Here is a short snippet of how to use it:
```rust
let mut sudoku: Sudoku = Sudoku::from_string("004080300000003042800405907302070508050000070608090201406207009520900000007010400");
```
### `draw()`
The `draw()` method simply draws the current state of the board in the console.
```rust
sudoku.draw();
```
Here would be the output for the example sudoku string that was used in the `from_string()` section:
```
+-----------------------------+    
|       4 |    8    | 3       |    
|         |       3 |    4  2 |    
| 8       | 4     5 | 9     7 |    
|---------+---------+---------|    
| 3     2 |    7    | 5     8 |    
|    5    |         |    7    |    
| 6     8 |    9    | 2     1 |    
|---------+---------+---------|    
| 4     6 | 2     7 |       9 |    
| 5  2    | 9       |         |    
|       7 |    1    | 4       |    
+-----------------------------+ 
```
### `solve()`
This method simply tries its best to solve the board. The strategies it uses to do so are
explained in the `Explanations` section.
```rust
sudoku.solve()
```
### `is_solved()`
Returns a boolean. The boolean is `true` if the board is solved and `false` if not. It simply
checks for duplicate numbers in every row, column and 3x3 grid of the board.
```rust
let solved: bool = sudoku.is_solved();
```

## Explanations
This chapter will cover how the solver works and what kind of strategies it uses to solve
a sudoku board.
### The blacklist
Internally, the solver uses a blacklist to determine what number can and cannot be in a cell.
The blacklist is simply a 3D 9 by 9 by 9 array. Each cell of the board has one 9 digit long array
attributed. At each iteration, the solver tries to fill up each array of each cell as much as possible.
The more the blacklist is filled, the easier it gets for the solver to determine which number goes
in which cell. Here is an example of how the blacklist is used internally:
1. The algorithm looks at the first cells blacklist: `blacklist[0][0]`
2. The output is an array of the numbers that cannot be placed in the first cell: `[1, 2, 3, 4, 0, 6, 7, 8, 9]`
3. The algorith determines that only the number `5` can be placed in the first cell
### Main loop
The main loop iterates until the sudoku is solved and performs 3 main steps on each iteration: 
1. The solver will iterate over every cell and first do 3 basic checks and then 3 more complex checks
to fill the cells blacklists and numbers. 
2. Then it will go through every cells blacklist and see if there are is only one missing number to it, 
which would mean that that number can be inserted in this cell. If that is the case, the number is inserted.
3. Finally we check if any changes have been made while in this main-loop iteration. A change occurs if the 
blacklist has been changed or if a number was placed on the board in this iteration. If this did not happen, 
the checks aren't enough for this board and emergency backtracking is required.
### First 3 basic checks
These 3 checks basically add every number that is in a cells region to its blacklist. We define a cells region as all the cells
that are in the same row, column and 3x3 grid of the current cell. 
1. First basic check looks at the current cells row and adds every number of this row to the blacklist of the cell
2. Second basic check looks at the current cells column and adds every number of this column to the blacklist of the cell
3. Third basic check looks at the current cells 3x3 grid and adds every number of this 3x3 grid to the blacklist of the cell
### 3 more complex checks
These checks fill the board if certain conditions regarding the blacklist are met.
While iterating through the cells of the board, we compute the cells whitelist and then iterate over the numbers that
could fit in this cell. For simplicity we will call the number from the whitelist in an iteration `X`. 
In this iteration we do the following checks:
1. Check if all the other cells in this current row have `X` in their blacklist. If they do, `X` can only be placed here.
```
+-----------------------------+                 +-----------------------------+
|         |    9  7 |       6 |                 |         |    9  7 |       6 |
| 5       | 2       | 1     4 |                 | 5    *9*| 2       | 1     4 |
| 3       |       1 |    7    |                 | 3       |       1 |    7    |
|---------+---------+---------|                 |---------+---------+---------|
|    9  3 | 8  2  5 |       7 |                 |    9  3 | 8  2  5 |       7 |
|         | 9  1  4 |         | ----becomes---> |         | 9  1  4 |         |
| 4       | 7  3  6 | 5  9    |                 | 4       | 7  3  6 | 5  9    |
|---------+---------+---------|                 |---------+---------+---------|
|    4    | 1       |       9 |                 |    4    | 1       |       9 |
| 8     2 |       9 |       1 |                 | 8     2 |       9 |       1 |
| 9       | 6  4    |         |                 | 9       | 6  4    |         |
+-----------------------------+                 +-----------------------------+
```
2. Check if all the other cells in this current column have `X` in their blacklist. If they do, `X` can only be placed here.
```
+-----------------------------+                 +-----------------------------+
| 1       |    9  7 |       6 |                 | 1       |    9  7 |       6 |
| 5     9 | 2       | 1     4 |                 | 5 *7* 9 | 2       | 1     4 |
| 3       |       1 | 9  7    |                 | 3       |       1 | 9  7    |
|---------+---------+---------|                 |---------+---------+---------|
| 6  9  3 | 8  2  5 | 4  1  7 |                 | 6  9  3 | 8  2  5 | 4  1  7 |
| 2     7 | 9  1  4 |         | ----becomes---> | 2     7 | 9  1  4 |         |
| 4       | 7  3  6 | 5  9    |                 | 4       | 7  3  6 | 5  9    |
|---------+---------+---------|                 |---------+---------+---------|
| 7  4    | 1       |       9 |                 | 7  4    | 1       |       9 |
| 8     2 |    7  9 |    4  1 |                 | 8     2 |    7  9 |    4  1 |
| 9       | 6  4    | 7       |                 | 9       | 6  4    | 7       |
+-----------------------------+                 +-----------------------------+
```
3. Check if all the other cells in this current 3x3 grid have `X` in their blacklist. If they do, `X` can only be placed here.
```
+-----------------------------+                 +-----------------------------+
|    4  5 | 2  8    | 7  9  6 |                 |    4  5 | 2  8    | 7  9  6 |
|         |       4 | 1       |                 |         |       4 | 1       |
|       9 |         | 4     3 |                 |       9 |         | 4     3 |
|---------+---------+---------|                 |---------+---------+---------|
| 9       | 7       | 5  6    |                 | 9       | 7       | 5  6    |
|    8    | 5  1    | 9  4  7 | ----becomes---> |*6* 8    | 5  1    | 9  4  7 |
| 7  5    |       9 |       1 |                 | 7  5    |       9 |       1 |
|---------+---------+---------|                 |---------+---------+---------|
| 4     6 |         | 2       |                 | 4     6 |         | 2       |
| 5  2  7 | 4       |         |                 | 5  2  7 | 4       |         |
|         |    2  5 | 6  7  4 |                 |         |    2  5 | 6  7  4 |
+-----------------------------+                 +-----------------------------+
```
### Emergency Backtracking
These checks are a big help and can solve most humanly solvable sudokus. A lot of time tho, these checks aren't enough!
In case we see that the checks didn't change anything to the board or to the blacklist, we resort to backtracking.
First we compute the best bet to take. For this we simply look for the cell with the least amount of possible numbers that
could be put in it. We then try to solve the board with the first possible number. We still try to only use logic after that.
The solver always keeps in mind that it has taken a bet and makes sure there are no errors on the board. If there are errors,
our bet was wrong, so we backtrack. We reset the board and blacklist to their initial state and try with the next possible number.
If after taking a bet we still can't solve the board with only logic, we take a new bet. We keep track of all our bets in a stack.
If the top bet only leads to errors, then we know that one of the bottom bets is wrong. If this happens, we pop the top bet from 
the stack and try with a new number.
