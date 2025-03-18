pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec4 {
    pub fn new() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn from(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 { x, y, z, w }
    }
}
