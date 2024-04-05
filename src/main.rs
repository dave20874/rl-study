// TODO: Introduce a Coord struct and use it instead of (usize, usize).
// TODO: Also make Coord the associated type State for GridWorld1.
// TODO: Introduce a Direction struct and use it instead of usize for actions.
// TODO: Make Direction the associated type Action for GridWorld1.

use std::{collections::HashMap, hash::Hash};


// A trait for a Dynamic Programming environment.
// This type of environment consists of:
//  * A set of states, identified with u32 values
//  * A set of actions, identified with u32 values (and a function giving valid actions for a state)
//  * A function p(s, a) -> Vec<(prob, sp, expected_r)> A list of next states and expected rewards, with an associated probability

trait DPEnv<S, A> 
    where S : Hash, A : Hash 
{
    // get states
    fn states(&self) -> &Vec<S>;

    // get actions (from this state)
    fn actions(&self, state: &S) -> &Vec<A>;

    // Dynamics: takes a state and action, returns a vector of probabilities for each expected reward and next state
    fn p(&self, s: &S, a: &A) -> &Vec<(f64, f64, S)>;
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Action {
    Up,
    Left,
    Down,
    Right,
}

// Coordinates are (0, 0) in the top, left.
// x increases to the right.
// y increases downward.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

// Grid World for Exmple 3.5 of Sutton and Barto
struct GridWorld3_5 {
    states_vec: Vec<Coord>,
    actions_vec: Vec<Action>,

    // Mapping of (state, action) to vector of outcomes (probability, reward, next state)
    dynamics: HashMap<(Coord, Action), Vec<(f64, f64, Coord)>>,
}

impl GridWorld3_5 {
    const POINT_A: Coord = Coord{x:1, y:0};
    const POINT_A_PRIME: Coord = Coord{x:1, y:4};
    const POINT_B: Coord = Coord{x:3, y:0};
    const POINT_B_PRIME: Coord = Coord{x:3, y:2};
    const REWARD_DEFAULT: f64 = 0.0;
    const REWARD_A: f64 = 10.0;
    const REWARD_B: f64 = 5.0;
    const REWARD_OFF_GRID: f64 = -1.0;
    const GRID_X: i32 = 5;
    const GRID_Y: i32 = 5;

    fn new() -> GridWorld3_5 {

        let mut states_vec: Vec<Coord> = Vec::new();
        for y in 0..Self::GRID_Y {
            for x in 0..Self::GRID_X {
                states_vec.push(Coord {x, y});
            }
        }

        let mut actions_vec: Vec<Action> = Vec::from([Action::Up, Action::Down, Action::Left, Action::Right]);

        let mut dynamics: HashMap<(Coord, Action), Vec<(f64, f64, Coord)>> = HashMap::new();
        for s in &states_vec {

            // initialize next_state, reward to defaults
            let mut next_state = *s;
            let mut reward = Self::REWARD_DEFAULT;

            for a in &actions_vec {
                if *s == Self::POINT_A {
                    // reward is REWARD_A
                    // next state is A'
                    reward = Self::REWARD_A;
                    next_state = Self::POINT_A_PRIME;
                }
                else if *s == Self::POINT_B {
                    // reward is REWARD_B
                    // next state is B'
                    reward = Self::REWARD_B;
                    next_state = Self::POINT_B_PRIME;
                }
                else {
                    match a {
                        Action::Up => {
                            // Try to move up
                            next_state.y -= 1;
                        }
                        Action::Down => {
                            // Try to move down
                            next_state.y += 1;
                        }
                        Action::Left => {
                            // Try to move left
                            next_state.x -= 1;
                        }
                        Action::Right => {
                            // Try to move right
                            next_state.x += 1;
                        }
                    }

                    // if new position is off grid, stay in place, reward is -1
                    if next_state.x < 0 || next_state.x >= Self::GRID_X ||
                       next_state.y < 0 || next_state.y >= Self::GRID_Y {
                        // reward is -1
                        // Stay in the same place
                        reward = Self::REWARD_OFF_GRID;
                        next_state = *s;
                    }
                }

                // This MDP is deterministic, only one outcome per state/action, with probability 1.0
                let mut outcome: Vec<(f64, f64, Coord)> = Vec::new();
                outcome.push( (1.0, reward, next_state));

                // Map (s, a) to outcome
                dynamics.insert((*s, *a), outcome);
            }
        }

        GridWorld3_5 { states_vec, actions_vec, dynamics}
    }
}

impl DPEnv<Coord, Action> for GridWorld3_5 {
    fn states(&self) -> &Vec<Coord> {
        &self.states_vec
    }

    fn actions(&self, state: &Coord) -> &Vec<Action> {
        &self.actions_vec
    }

    fn p(&self, s: &Coord, a: &Action) -> &Vec<(f64, f64, Coord)> {
        self.dynamics.get(&(*s, *a)).unwrap()
    }
}

// A trait for a solver of DPEnv environments
struct DPSolution<S, A> 
    where S: Hash, A: Hash
{
    values: HashMap<S, f64>,  // map state to value
    policy: HashMap<S, A>,    // map state to best action
}

impl<S, A> DPSolution<S, A> where S: Hash, A: Hash {
    fn solve(env: &dyn DPEnv<S, A>) -> DPSolution<S, A> {

        let mut values: HashMap<S, f64> = HashMap::new();  // map state to value
        let mut policy: HashMap<S, A> = HashMap::new();      // map state to best action

        // TODO : Implement policy iteration and find state values and optimal policy

        DPSolution {values, policy}
    }
}

fn main() {
    // create Dynamic Programming environment.
    let env = GridWorld3_5::new();

    // Compute state values and optimal policy
    let soln = DPSolution::solve(&env);

    // Print state values
    // Print optimal policy

    println!("Hello, world!");
}
