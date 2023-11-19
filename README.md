# Sudoku Solver
## Motivation
I love sudoku! I like to solve puzzles on the Pink Pointer Sudoku App when I'm bored. Seeing how hard
the Hard and Evil puzzles are, I was wondering if a basic sudoku solver using human 
strategies would be able to solve it without using backtracking. This also proved to be 
a prefect first relatively big project I can make using Rust, as I am currently learning the language!

## Goals and Accomplishments
The initial goal of this project was to make a sudoku solver that uses human strategies to
solve the hardest puzzles in the Pink Pointer Sudoku App. The App has four difficulty stages:
Easy, Medium, Hard and Evil. I copied the first 3 puzzles from each category for testing.
As of this moment, the solver is capable of solving all but the Evil category in around 2ms
per puzzle.

I also looked into how fast other solvers are online and stumbled upon 
[this small challenge on StackExchange](https://codegolf.stackexchange.com/questions/190727/the-fastest-sudoku-solver).
The goal of this challenge is to use an algorithm to solve a given 17-clue board. I've copied
the board to the `challenge.sdku` board file. The official winner of the challenge used Tdoku and
got a solving time of 201ms! This solver managed to solve the challenge with an average time of 2ms :)

The hardest challenge I've set for myself is to make the solver be able to solve the "World's most difficult
Sudoku" designed by Arto Inkala in 2012. The puzzle is featured and talked about
[here](https://abcnews.go.com/blogs/headlines/2012/06/can-you-solve-the-hardest-ever-sudoku) and 
[here](https://sudoku2.com/play-the-hardest-sudoku-in-the-world/). I've copied the board to the
`inkala.sdku` board file. As of this moment, the solver isn't able to place a single digit in any cell :D

## TODOs
- Finish README
- Upgrade solver for Evil boards
- Add a new check for "shadow blacklists"
- Add a backtracking method as a last resort if logic isnt enough
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
    let mut sudoku: Sudoku = Sudoku::load("boards/challenge.sdku"); // Load a sudoku board by passing the .sdku file path

    sudoku.draw(); // Draw the current state of the board on the console

    println!("{}", sudoku.is_solved()); // Print wether the board is solved or not

    let start: Instant = Instant::now(); // Measure time it takes to solve
    sudoku.solve(); // Solve the board
    println!("Time elapsed to solve sudoku: {:?}", start.elapsed()); // Print time it took to solve

    sudoku.draw(); // Draw the solved board

    println!("{}", sudoku.is_solved()); // Should print 'true' now!
}
```
### `load()`
Returns a `Sudoku` struct. Requires a path to a `.sdku` file. A `.sdku` file is
a representation of a 9 by 9 sudoku board. The file must contain comma-separated
digits, each representing a cell. An empty cell is represented by a 0. Here is a 
short snippet of how to use it:
```rust
let mut sudoku: Sudoku = Sudoku::load("path/to/file.sdku");
```
Here is an example of a `.sdku` file and its format:
```
0, 0, 0, 7, 0, 0, 0, 0, 0,
1, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 4, 3, 0, 2, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 6,
0, 0, 0, 5, 0, 9, 0, 0, 0,
0, 0, 0, 0, 0, 0, 4, 1, 8,
0, 0, 0, 0, 8, 1, 0, 0, 0,
0, 0, 2, 0, 0, 0, 0, 5, 0,
0, 4, 0, 0, 0, 0, 3, 0, 0,
```
There is probably an infinite amount of ways to break the `load()` method by inputing
some bogus format, as I haven't fully fleshed out the method, so be careful and kind with it ;)
### `draw()`
The `draw()` method simply draws the current state of the board in the console.
```rust
sudoku.draw();
```
Here would be the output for the example `.sdku` file that was shown in the `load()` section:
```
+-----------------------------+
|         | 7       |         |
| 1       |         |         |
|         | 4  3    | 2       |
|---------+---------+---------|
|         |         |       6 |
|         | 5     9 |         |
|         |         | 4  1  8 |
|---------+---------+---------|
|         |    8  1 |         |
|       2 |         |    5    |
|    4    |         | 3       |
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
