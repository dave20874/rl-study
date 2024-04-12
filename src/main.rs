// TODO: Introduce a Coord struct and use it instead of (usize, usize).
// TODO: Also make Coord the associated type State for GridWorld1.
// TODO: Introduce a Direction struct and use it instead of usize for actions.
// TODO: Make Direction the associated type Action for GridWorld1.

mod dist_env;
mod gridworld_3_5;
mod dp_solution;

use std::{collections::HashMap, hash::Hash};
use dist_env::DistEnv;
use gridworld_3_5::GridWorld3_5;
use dp_solution::DPSolution;

fn main() {
    // create Dynamic Programming environment.
    let env = GridWorld3_5::new();

    // Compute state values and optimal policy
    let soln = DPSolution::solve(&env);

    // Print state values
    // Print optimal policy

    println!("Hello, world!");
}
