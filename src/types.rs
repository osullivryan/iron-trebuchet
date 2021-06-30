pub struct TrebuchetParameters {
    pub l_arm_lo: f64,
    pub l_arm_sh: f64,
    pub l_arm_we: f64,
    pub l_sling: f64,
    pub h_pivot: f64,
    pub cw_mass: f64,
    pub cw_i: f64,
    pub arm_mass: f64,
    pub arm_i: f64,
    pub l_arm_cg: f64,
    pub rel_angle: f64,
}

pub struct ProjectileParameters {
    pub proj_mass: f64,
    pub proj_dia: f64,
    pub cd: f64,
    pub area: f64,
}

pub struct GlobalParameters {
    pub g: f64,
    pub wind_v: f64,
    pub ro: f64,
}

pub struct SimulationEnvironment {
    pub trebuchet: TrebuchetParameters,
    pub projectile: ProjectileParameters,
    pub global: GlobalParameters,
}

pub struct SimulationFunction {
    pub evaluation_function: fn(f64, Vec<64>, SimulationEnvironment) -> Vec<64>,
    pub integration_stop_function: fn(Vec<64>, SimulationEnvironment) -> bool,
}