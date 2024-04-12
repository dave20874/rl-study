use crate::dist_env::DistEnv;
use std::collections::HashMap;

// A trait for a solver of DistEnv environments
pub struct DPSolution<T: DistEnv>
{
    values: HashMap<DistEnv::S, f64>,  // map state to value
    policy: HashMap<DistEnv::S, DistEnv::A>,    // map state to best action
}

impl<T: DistEnv> DPSolution<T> {
    fn solve(env: &dyn DistEnv) -> DPSolution<S, A> {

        let mut values: HashMap<S, f64> = HashMap::new();  // map state to value
        let mut policy: HashMap<S, A> = HashMap::new();      // map state to best action

        // TODO : Implement policy iteration and find state values and optimal policy
  
        DPSolution {values: values, policy: policy}
    }
}
