use crate::types::{SimulationFunction, SimulationEnvironment};

pub struct Simulation {
    functions: SimulationFunction,
    environments: SimulationEnvironment,
}

impl Solve for Simulation {
    fn rk45(&self, from: f64, h: f64, to: f64, max_h: f64, initial_values: &Vec<f64>) -> Vec<Vec<f64>> {
        return rk45(
            from,
            h,
            to,
            max_h,
            initial_values,
            &self.functions.evaluation_function,
            &self.functions.integration_stop_function,
            &self.environments,
        );
    }
}

fn rk45(
    from: f64,
    h: f64,
    to: f64,
    max_h: f64,
    y: &Vec<f64>,
    func: &fn(f64, Vec<f64>, SimulationEnvironment) -> Vec<f64>,
    stop_fn: &fn(Vec<f64>, SimulationEnvironment) -> bool,
    simulation_environment: &SimulationEnvironment,
) -> Vec<Vec<f64>> {
    let eps = 0.000001;
    let n_vars = y.len() as u32;

    let mut y_slice = vec![vec![0.0; (n_vars + 1) as usize]];


}