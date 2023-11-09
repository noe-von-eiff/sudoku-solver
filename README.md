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
- Implement a Benchmarking Function
- Some lookup tables to avoid division at every iteration

## Usage
TODO: How to use the few methods. How to structure a sdku file etc.

## Explanations
TODO: How the checks work and graphical representations of some patterns
