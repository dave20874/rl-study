use std::hash::Hash;

// A trait for a Distribution-based environment.
// This type of environment consists of:
//  * A set of states, identified with u32 values
//  * A set of actions, identified with u32 values (and a function giving valid actions for a state)
//  * A function p(s, a) -> Vec<(prob, sp, expected_r)> A list of next states and expected rewards, with an associated probability

pub trait DistEnv
{
    type S: Hash;
    type A: Hash;

    // get states
    fn states(&self) -> &Vec<Self::S>;

    // get actions (from this state)
    fn actions(&self, state: &Self::S) -> &Vec<Self::A>;

    // Dynamics: takes a state and action, returns a vector of probabilities for each expected reward and next state
    fn p(&self, s: &Self::S, a: &Self::A) -> &Vec<(f64, f64, Self::S)>;
}
