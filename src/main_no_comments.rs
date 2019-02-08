use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

pub struct State {
    is: Vec<isize>,
    cost: isize,
    g: isize,
    h: isize,
    parent: Option<Rc<State>>,
    kind: String, 
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.cost == other.cost
    }
}
impl Eq for State {}

pub struct Problem {
    state: Rc<State>,
    goal_state: Vec<isize>,
    visited: HashSet<Vec<isize>>,
    under_consideration: BinaryHeap<Rc<State>>,
    no_generated: isize,
    no_expanded: isize,
    heuristic: String
}

impl Problem {
    pub fn new(init_state: Vec<isize>, goal_state: Vec<isize>) -> Problem {
        Problem {
            state: Rc::new(State {
                is: init_state,
                cost: 0,
                g: 0,
                h: 999,
                parent: None,
                kind: String::from("None"),
            }),
            goal_state,
            visited: HashSet::new(),
            under_consideration: BinaryHeap::new(),
            no_generated: 0,
            no_expanded: 0,
            heuristic: String::from("Manhattan")
        }
    }
    pub fn expand(&mut self) -> Vec<Rc<State>> {
        let loc = self.state.is.iter()
            .position(|&x| x == 0)
            .expect("Cannot find the blank in current state.");

        let mut possible_states = Vec::with_capacity(4);
        self.no_expanded += 1; // incrementing

        macro_rules! do_the_needful {
            ($new_loc:expr, $kind:expr) => {
                let mut new_state = self.state.is.clone();
                new_state.swap(loc, $new_loc);

                if !self.visited.contains(&new_state) {
                    let mut cost = 0;
                    let new = vec![&self.state.is[0..3], &self.state.is[3..6], &self.state.is[6..9]].clone();
                    let goal = vec![&self.goal_state[0..3], &self.goal_state[3..6], &self.goal_state[6..9]];

                    for g in self.goal_state.iter() {
                            let (goal_x, goal_y) = find(&goal, g);
                            let (new_x, new_y) = find(&new, g);
                            cost += if self.heuristic == "Manhattan" {
                                isize::abs(goal_x - new_x) + isize::abs(goal_y - new_y)
                            } else {
                                if goal_x == new_x && goal_y == new_y {0} else {1}
                            };
                    }

                    let state = State {
                        is: new_state,
                        cost: self.state.g + 1 + cost,
                        g: self.state.g + 1,
                        h: cost,
                        parent: Some(self.state.clone()),
                        kind: String::from($kind),
                    };
                    possible_states.push(Rc::new(state));
                    self.no_generated += 1; // incrementing
                }
            };
        }

        match loc { // going up
            0...2 => (),
            i => {do_the_needful!(i - 3, "Up");}
        };

        match loc { // going down
            6...8 => (),
            i => {do_the_needful!(i + 3, "Down");}
        };

        match loc {
            0 | 3 | 6 => (), // going left
            i => {do_the_needful!(i - 1, "Left");}
        };

        match loc { // going right
            2 | 5 | 8 => (),
            i => {do_the_needful!(i + 1, "Right");}
        };

        possible_states // returning the vector
    }

    pub fn solve(&mut self, heuristic: &str) {
        println!("{:?}", self);
        self.heuristic = String::from(heuristic);
        println!("Solving using {} distance...", self.heuristic);

        loop {
            let possible_states = self.expand();

            self.under_consideration
                .append(&mut BinaryHeap::from(possible_states));
            let next_state = self.under_consideration.pop().expect("Reached a dead end.");
            self.visited.insert(next_state.is.clone());
            self.state = next_state;
            if self.state.h == 0 {
                println!(
                    "Expanded {} nodes.\nGenerated {} nodes.",
                    self.no_expanded, self.no_generated
                );
                break;
            }
        }
        self.trace_soln();
    }

    pub fn trace_soln(&self) {
        let mut soln: Vec<String> = vec![];
        let mut parent = self.state.parent.clone();
        
        while let Some(p) = parent {
            soln.push((*p).kind.clone());
            parent = (*p).parent.clone();
        }

        soln.reverse();
        println!("Solution is {:?}", &soln[1..]);
    }
}

fn main() {
    let mut problem_1 = Problem::new(
        vec![1, 2, 3, 7, 4, 5, 6, 8, 0],
        vec![1, 2, 3, 8, 6, 4, 7, 5, 0],
    );

    println!("\n\nProblem 1");
    problem_1.solve("Manhattan");

    let mut problem_1 = Problem::new(
        vec![1, 2, 3, 7, 4, 5, 6, 8, 0],
        vec![1, 2, 3, 8, 6, 4, 7, 5, 0],
    );

    problem_1.solve("Hamming");
    
    let mut problem_2 = Problem::new(
        vec![2, 8, 1, 3, 4, 6, 7, 5, 0],
        vec![3, 2, 1, 8, 0, 4, 7, 5, 6],
    );

    println!("\n\nProblem 2");
    problem_2.solve("Manhattan");

    let mut problem_2 = Problem::new(
        vec![2, 8, 1, 3, 4, 6, 7, 5, 0],
        vec![3, 2, 1, 8, 0, 4, 7, 5, 6],
    );

    problem_2.solve("Hamming");

    let mut problem_3 = Problem::new(
        vec![0, 1, 3, 4, 2, 5, 7, 8, 6],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
    );

    println!("\n\nProblem 3");
    problem_3.solve("Manhattan");

    let mut problem_3 = Problem::new(
        vec![0, 1, 3, 4, 2, 5, 7, 8, 6],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
    );

    problem_3.solve("Hamming");
    
    let mut problem_4 = Problem::new(
        vec![0, 3, 1, 4, 2, 5, 7, 8, 6],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
    );

    println!("\n\nProblem 4");
    problem_4.solve("Manhattan");

    let mut problem_4 = Problem::new(
        vec![0, 3, 1, 4, 2, 5, 7, 8, 6],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
    );

    problem_4.solve("Hamming");
}

pub fn find(_vec: &Vec<&[isize]>, r: &isize) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;
    for (i, row) in _vec.iter().enumerate() {
        if row.contains(&r) {
            x = i;
        }
        for (j, val) in row.iter().enumerate() {
            if *val == *r {
                y = j;
            }
        }
    }
    (x as isize, y as isize)
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-------------\n| {} | {} | {} |\n| {} | {} | {} |\n| {} | {} | {} |\n-------------", 
        self.is[0], self.is[1], self.is[2], 
        self.is[3], self.is[4], self.is[5],
        self.is[6], self.is[7], self.is[8])
    }
}

impl fmt::Debug for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current State:             Goal State:\n-------------             -------------\n| {} | {} | {} |             | {} | {} | {} | \n| {} | {} | {} |             | {} | {} | {} |\n| {} | {} | {} |             | {} | {} | {} |\n-------------             -------------", 
        self.state.is[0], self.state.is[1], self.state.is[2], self.goal_state[0], self.goal_state[1], self.goal_state[2], 
        self.state.is[3], self.state.is[4], self.state.is[5], self.goal_state[3], self.goal_state[4], self.goal_state[5],
        self.state.is[6], self.state.is[7], self.state.is[8], self.goal_state[6], self.goal_state[7], self.goal_state[8])
    }
}
