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
the challenge used Tdoku and got a solving time of 201ms for all the 49k boards! This solver manages to solve one 
board from that list with an average time of 2ms. The solver isn't able to solve them all yet, due to a memory overflow
issue I still need to fix.

The hardest challenge I've set for myself is to make the solver be able to solve the "World's most difficult
Sudoku" designed by Arto Inkala in 2012. The puzzle is featured and talked about
[here](https://abcnews.go.com/blogs/headlines/2012/06/can-you-solve-the-hardest-ever-sudoku) and 
[here](https://sudoku2.com/play-the-hardest-sudoku-in-the-world/). This proved quite challenging because the logic
strategies I implemented simply aren't enough, so I had to add a last resort backtracking algorithm. If logic isn't
enough, the solver takes a bet on a cell and tries different possible numbers. With the backtracking algorithm
implemented, the solver solves the Inkala board in about 100ms!

## TODOs
- Finish README
- Some lookup tables to avoid division at every iteration
- Write tests

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
what the blacklist is and how it is inteded to be used. say that each iteration tries to write
as much as possible in the blacklist, bc the more it is filled up that better the solver can do its work!
### Main loop
The main loop iterates until the sudoku is solved. Uses the util is_solved() method for this. Main loop
performs 3 main steps: it will iterate over every cell and first do 3 basic checks and then 3 more complex checks
to fill the cells blacklists and numbers. Secondly it will go through every cells blacklist and see if there
are is only one missing number to it, which would mean that that number can be inserted in this cell. The third and
last step that it performs is to check if any changes have been made while in this main-loop iteration. A change occurs
if the blacklist has been changed in this iteration or if a number was placed on the board in this iteration. If this did
not happen, the checks aren't enough for this board and some smart emergency backtracking is required.
### First 3 basic checks
--> Add as vocabulary that a cells so called "region" is considered to be the cells in the row, columns and 3x3 grid of the
current cell. Maybe add a graphic of what a cells region is.
These 3 checks basically add every number that is in a cells region to its blacklist.
1. First basic check looks at the current cells row and adds every number of this row to the blacklist of the cell
2. Second basic check looks at the current cells column and adds every number of this column to the blacklist of the cell
3. Third basic check looks at the current cells 3x3 grid and adds every number of this 3x3 grid to the blacklist of the cell
### 3 more complex checks
These checks fill the board if certain conditions regarding the blacklist are met
Add graphs on for these checks for better visualization
First we iterate through every cell and at each cell, we compute its whitelist and then iterate over the numbers that
could fit in this cell. For simplicity we will call the number from the whitelist in an iteration `X`. 
In this iteration we do the following checks:
1. Check if all the other cells in this current row have the whitelist number we are looking at in their blacklist.
2. Check if all the other cells in this current column have the whitelist number we are looking at in their blacklist.
3. Check if all the other cells in this current 3x3 grid have the whitelist number we are currently looking at in their blacklist.
If all cell-blacklists in the current cells row, column or 3x3 grid contain a number that is in this cells whitelist,
that means that this is the only place where the number can be placed!
### Emergency Backtracking
These checks are a big help and can solve most humanly solvable sudokus. A lot of time tho, these checks aren't enough!
In case we see that the checks didn't change anything to the board or to the blacklist, we so an emergency backtracking.
First we compute the best bet to take. It is a numerical value of every cell and how much changing that cell would affect
other cells. We want our change to affect a lot of surrounding cells, so we can quickly determine if the bet we took was 
correct or not and backtrack in case it wasn't. backtracking is not yet done so I can't say more rn.
