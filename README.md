# 8 Puzzle Problem

The 8-puzzle problem is a puzzle invented and popularized by Noyes Palmer Chapman in the 1870s.
It is played on a 3-by-3 grid with 8 square blocks labeled 1 through 8 and a blank square.
Your goal is to rearrange the blocks so that they are in order.
You are permitted to slide blocks horizontally or vertically into the blank square.
The following shows a sequence of legal moves from an initial board position (left) to the goal position (right).

```
   1  3        1     3        1  2  3        1  2  3        1  2  3
4  2  5   =>   4  2  5   =>   4     5   =>   4  5      =>   4  5  6
7  8  6        7  8  6        7  8  6        7  8  6        7  8
initial                                                      goal
```

## Problem Formulation

- **Goal**: Pieces end up in locations as described by the goal state.
- **States**: All possible permutations of the puzzle.
- **Actions**: Move the blank Up, Down, Left, or Right.
- **Performance Measure**: Number of total moves in the solution, if the solution exists.

## Problem Structure

The program is composed of 3 major things. 2 structs `Problem` and `State`, and a `main` method.
The problem state exposes a `solve` method, which is all that is needed to find a solution.
More detailed information can be found by clicking on the respective struct's name further down this page.

## Global Variables

There are no global variables in this implementation.

## Methods

Further information on implemented methods can be found by going to the respective struct's page via the links below.

## Examples

### Case 1:

```
Initial State:             Goal State:
-------------             -------------
| 1 | 2 | 3 |             | 1 | 2 | 3 |
| 7 | 4 | 5 |             | 8 | 6 | 4 |
| 6 | 8 | 0 |             | 7 | 5 | 0 |
-------------             -------------
Solving using Manhattan distance...
Expanded 27 nodes.
Generated 47 nodes.
Solution is ["Up", "Left", "Down", "Left", "Up", "Right", "Down"]
Initial State:             Goal State:
-------------             -------------
| 1 | 2 | 3 |             | 1 | 2 | 3 |
| 7 | 4 | 5 |             | 8 | 6 | 4 |
| 6 | 8 | 0 |             | 7 | 5 | 0 |
-------------             -------------
Solving using Hamming distance...
Expanded 18 nodes.
Generated 34 nodes.
Solution is ["Up", "Left", "Down", "Left", "Up", "Right", "Down"]
```

### Case 2:

```
nitial State:             Goal State:
-------------             -------------
| 2 | 8 | 1 |             | 3 | 2 | 1 |
| 3 | 4 | 6 |             | 8 | 0 | 4 |
| 7 | 5 | 0 |             | 7 | 5 | 6 |
-------------             -------------
Solving using Manhattan distance...
Expanded 8 nodes.
Generated 16 nodes.
Solution is ["Up", "Left", "Up", "Left", "Down"]

Initial State:             Goal State:
-------------             -------------
| 2 | 8 | 1 |             | 3 | 2 | 1 |
| 3 | 4 | 6 |             | 8 | 0 | 4 |
| 7 | 5 | 0 |             | 7 | 5 | 6 |
-------------             -------------
Solving using Hamming distance...
Expanded 7 nodes.
Generated 15 nodes.
Solution is ["Up", "Left", "Up", "Left", "Down"]
```

### Case 3:

```
Initial State:             Goal State:
-------------             -------------
| 0 | 1 | 3 |             | 1 | 2 | 3 |
| 4 | 2 | 5 |             | 4 | 5 | 6 |
| 7 | 8 | 6 |             | 7 | 8 | 0 |
-------------             -------------
Solving using Manhattan distance...
Expanded 5 nodes.
Generated 11 nodes.
Solution is ["Right", "Down", "Right"]

Initial State:             Goal State:
-------------             -------------
| 0 | 1 | 3 |             | 1 | 2 | 3 |
| 4 | 2 | 5 |             | 4 | 5 | 6 |
| 7 | 8 | 6 |             | 7 | 8 | 0 |
-------------             -------------
Solving using Hamming distance...
Expanded 4 nodes.
Generated 10 nodes.
Solution is ["Right", "Down", "Right"]
```

### Case 4:

```
Initial State:             Goal State:
-------------             -------------
| 0 | 3 | 1 |             | 1 | 2 | 3 |
| 4 | 2 | 5 |             | 4 | 5 | 6 |
| 7 | 8 | 6 |             | 7 | 8 | 0 |
-------------             -------------
Solving using Manhattan distance...
thread "main" panicked at "Reached a dead end.", src\libcore\option.rs:1038:5
error: process didn't exit successfully: `target\debug\project_1.exe` (exit code: 101)
```
