use crate::types::SimulationEnvironment;
use crate::utilities::{zeros2d, zeros1d};

pub fn proj_xy(y: &Vec<Vec<f64>>, simulation_environment: SimulationEnvironment) -> Vec<Vec<f64>> {
    let mut xy = zeros2d(y.len() as u32, 3);

    for i in 0..y.len() {
        xy[i][0] = y[i][0];
        xy[i][1] = -1.0 * simulation_environment.trebuchet.l_arm_lo * y[i][1].sin() - simulation_environment.trebuchet.l_sling * (y[i][1] + y[i][5]).sin();
        xy[i][2] = simulation_environment.trebuchet.l_arm_lo * y[i][1].cos() + simulation_environment.trebuchet.l_sling * (y[i][1] + y[i][5]).cos();
    }
    return xy;
}

pub fn proj_v(y: &Vec<f64>, simulation_environment: SimulationEnvironment) -> Vec<f64> {
    let xp = -1.0 * simulation_environment.trebuchet.l_arm_lo * y[0].sin() - simulation_environment.trebuchet.l_sling * (y[0] + y[4]).sin();
    let xv = -1.0 * simulation_environment.trebuchet.l_sling * (y[0] + y[4]).cos() * (y[1] + y[5]) - simulation_environment.trebuchet.l_arm_lo * y[0].cos() * y[1];
    let yp = 1.0 * simulation_environment.trebuchet.l_arm_lo * y[0].cos() + simulation_environment.trebuchet.l_sling * (y[0] + y[4]).cos();
    let yv = -1.0 * simulation_environment.trebuchet.l_sling * (y[0] + y[4]).sin() * (y[1] + y[5]) - simulation_environment.trebuchet.l_arm_lo * y[0].sin() * y[1];
    return vec![xp, xv, yp, yv];
}

pub fn cw_xy(y: &Vec<Vec<f64>>, simulation_environment: SimulationEnvironment) -> Vec<Vec<f64>> {
    let mut xy = zeros2d(y.len() as u32, 3);

    for i in 0..y.len() {
        xy[i][0] = y[i][0];
        xy[i][1] = simulation_environment.trebuchet.l_arm_sh*y[i][1].sin() + simulation_environment.trebuchet.l_arm_we*(y[i][1]+y[i][3]).sin();
        xy[i][2] = -1.0 * simulation_environment.trebuchet.l_arm_sh*y[i][1].cos() - simulation_environment.trebuchet.l_arm_we*(y[i][1]+y[i][3]).cos();
    }
    return xy;
}

pub fn arm_sl_xy(y: &Vec<Vec<f64>>, simulation_environment: SimulationEnvironment) -> Vec<Vec<f64>> {
    let mut xy = zeros2d(y.len() as u32, 3);
    for i in 0..y.len() {
        xy[i][0] = y[i][0];
        xy[i][1] = -simulation_environment.trebuchet.l_arm_lo * y[i][1].sin();
        xy[i][2] = simulation_environment.trebuchet.l_arm_lo * y[i][1].cos();
    }
    return xy;

}

pub fn weight_arm_xy(y: &Vec<Vec<f64>>, simulation_environment: SimulationEnvironment) -> Vec<Vec<f64>> {
    let mut xy = zeros2d(y.len() as u32, 3);
    for i in 0..y.len() {
        xy[i][0] = y[i][0];
        xy[i][1] = simulation_environment.trebuchet.l_arm_sh * y[i][1].sin();
        xy[i][2] = -simulation_environment.trebuchet.l_arm_sh * y[i][1].cos();
    }
    return xy;

}
