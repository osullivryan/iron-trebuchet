use crate::types::SimulationEnvironment;
use crate::utilities::zeros1d;

pub fn phase_1(time: f64, y: &Vec<f64>, simulation_environment: SimulationEnvironment) -> Vec<f64> {
    let ret = zeros1d(y.len() as u32);
    let m11 = -1.0 * simulation_environment.projectile.proj_mass * simulation_environment.trebuchet.l_arm_lo.powi(2)
}