// TODO: Introduce a Coord struct and use it instead of (usize, usize).
// TODO: Also make Coord the associated type State for GridWorld1.
// TODO: Introduce a Direction struct and use it instead of usize for actions.
// TODO: Make Direction the associated type Action for GridWorld1.


// A trait for a Dynamic Programming environment.
// This type of environment consists of:
//  * A set of states, identified with u32 values
//  * A set of actions, identified with u32 values (and a function giving valid actions for a state)
//  * A function p(s, a) -> Vec<(prob, sp, expected_r)> A list of next states and expected rewards, with an associated probability

trait DPEnv {
    // get states
    fn states(&self) -> Vec<usize>;

    // get actions (from this state)
    fn actions(&self, state: usize) -> Vec<usize>;

    // Dynamics: takes a state and action, returns a vector of probabilities for each expected reward and next state
    fn p(&self, s: usize, a: usize) -> Vec<(f64, f64, usize)>;
}

// Grid World for example N
struct GridWorld1 {
    // TODO: Make GridWorld1 have the trait DPEnv
    dim: (usize, usize), // (x, y) or (width, height)
    start: (usize, usize), // (x, y)
    goal: (usize, usize), // (x, y)
    current: (usize, usize),

    states_vec: Vec<usize>,
    actions_vec: Vec<usize>,
}

impl GridWorld1 {
    fn new(width: usize, height: usize) -> GridWorld1 {
        let dim = (width, height);
        let start = (0, height-1);   // start in upper left
        let goal = (width-1, 0);     // goal in lower right

        let mut states_vec: Vec<usize> = Vec::new();
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                states_vec.push(self.pos_to_state((x, y)));
            }
        }

        let mut actions_vec: Vec<usize> = Vec::from([0, 1, 2, 3]);  // up: 0, left: 1, down: 2, right: 3

        GridWorld1 { dim, start, goal, current: start, states_vec, actions_vec}
    }

    fn pos_to_state(&self, pos: (usize, usize)) -> usize {
        // positions are given state numbers starting with 0 in lower left (0, 0), incrementing to the right
        // with +X.
        pos.0 + pos.1*self.width
    }
}

impl DPEnv for GridWorld1 {
    fn states(&self) -> &Vec<usize> {
        &self.states
    }

    fn actions(&self, state: usize) -> &Vec<usize> {
        &self.actions_vec
    }

    fn p(s: usize, a: usize) -> Vec<(float64, float64, usize)> {
        // TODO-DW : 
    }
}

// A trait for a solver of DPEnv environments
struct DPSolution<T> 
    where T : DPEnv {
    values: HashMap<u32, float64>;  // map state to value
    policy: HashMap<u32, u32>;      // map state to best action
    // TODO
}

impl DPSolution {
    fn solve(env: &DPEnv) -> DPSolution {
        // TODO : Implement policy iteration and find state values and optimal policy
        DPSolution { }
    }
}

fn main() {
    // create Dynamic Programming environment.
    env = GridWorld1.new();

    // Compute state values and optimal policy
    soln = DPSolution.solve(&env);

    // Print state values
    // Print optimal policy

    println!("Hello, world!");
}
