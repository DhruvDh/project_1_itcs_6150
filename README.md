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

## Structs

### `State`

A struct to encapsulate State information.

#### Properties

- `is`: A vector of integers, used to store the positions of numbers in the puzzle.
- `cost`: Cost of this state (g() + h())
- `g`: Cost to get here from the root.
- `h`: Estimated cost to the goal state.
- `kind`: The kind of action to be enacted to reach this state from the parent.

#### Further details

I am using Binary heap as a priority queue to choose the next state to move to.
The priority here is the "cost" in reverse order. To implement this I need to implement ordering trait for State.
This is done via implementing 4 traits - `Ord`, `PartialOrd`, `Eq` (or equality), `PartialEq`.
The ordering trait is implemented such that a State with a lower cost is considered greater than a state with a higher cost.

### `Problem`

A struct to encapsulate information about the current problem.

##### Properties

- `state`: A smart pointer to an instance of struct State.
- `goal_state`: A vector of integers defining the goal state.
- `visited`: A HashSet that stores all states that have already been visited.
- `under_consideration`: A BinaryHeap that keeps track of states we'll be choosing the next state from.
- `no_generated`: A counter to keep track of number of nodes generated.
- `no_expanded`: A counter to keep track of number of nodes expanded.
- `heuristic`: A string describing the current heuristic being used.

# Examples

### Case 1:

```
Current State:             Goal State:
-------------             -------------
| 1 | 2 | 3 |             | 1 | 2 | 3 |
| 7 | 4 | 5 |             | 8 | 6 | 4 |
| 6 | 8 | 0 |             | 7 | 5 | 0 |
-------------             -------------
Solving using Manhattan distance...
Expanded 19 nodes.
Generated 33 nodes.
Solution is ["Up", "Left", "Down", "Left", "Up", "Right", "Down", "Right"]

Current State:             Goal State:
-------------             -------------
| 1 | 2 | 3 |             | 1 | 2 | 3 |
| 7 | 4 | 5 |             | 8 | 6 | 4 |
| 6 | 8 | 0 |             | 7 | 5 | 0 |
-------------             -------------
Solving using Hamming distance...
Expanded 38 nodes.
Generated 63 nodes.
Solution is ["Up", "Left", "Down", "Left", "Up", "Right", "Down", "Right"]
```

### Case 2:

```
Current State:             Goal State:
-------------             -------------
| 2 | 8 | 1 |             | 3 | 2 | 1 |
| 3 | 4 | 6 |             | 8 | 0 | 4 |
| 7 | 5 | 0 |             | 7 | 5 | 6 |
-------------             -------------
Solving using Manhattan distance...
Expanded 14 nodes.
Generated 26 nodes.
Solution is ["Up", "Left", "Up", "Left", "Down", "Right"]

Current State:             Goal State:
-------------             -------------
| 2 | 8 | 1 |             | 3 | 2 | 1 |
| 3 | 4 | 6 |             | 8 | 0 | 4 |
| 7 | 5 | 0 |             | 7 | 5 | 6 |
-------------             -------------
Solving using Hamming distance...
Expanded 16 nodes.
Generated 28 nodes.
Solution is ["Up", "Left", "Up", "Left", "Down", "Right"]
```

### Case 3:

```
Current State:             Goal State:
-------------             -------------
| 0 | 1 | 3 |             | 1 | 2 | 3 |
| 4 | 2 | 5 |             | 4 | 5 | 6 |
| 7 | 8 | 6 |             | 7 | 8 | 0 |
-------------             -------------
Solving using Manhattan distance...
Expanded 9 nodes.
Generated 19 nodes.
Solution is ["Right", "Down", "Right", "Down"]

Current State:             Goal State:
-------------             -------------
| 0 | 1 | 3 |             | 1 | 2 | 3 |
| 4 | 2 | 5 |             | 4 | 5 | 6 |
| 7 | 8 | 6 |             | 7 | 8 | 0 |
-------------             -------------
Solving using Hamming distance...
Expanded 11 nodes.
Generated 20 nodes.
Solution is ["Right", "Down", "Right", "Down"]
```

### Case 4: (No solution)

```
Current State:             Goal State:
-------------             -------------
| 0 | 3 | 1 |             | 1 | 2 | 3 |
| 4 | 2 | 5 |             | 4 | 5 | 6 |
| 7 | 8 | 6 |             | 7 | 8 | 0 |
-------------             -------------
Solving using Manhattan distance...
thread 'main' panicked at 'Reached a dead end.', src\libcore\option.rs:1038:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: process didn't exit successfully: `target\debug\project_1.exe` (exit code: 101)
```
