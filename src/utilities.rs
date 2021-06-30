
pub fn zeros1d(size: u32) -> Vec<f64> {
    vec![0.0 size as usize]
}

pub fn zeros2d(size1: u32, size2: u32) -> Vec<Vec<f64>> {
    vec![vec![0.0; size2 as usize]; size1 as usize]
}
