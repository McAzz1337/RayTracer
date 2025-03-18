use crate::math::vec3::Vec3;

pub static CHROME: Material = Material::new(
    Vec3::from(0.0215, 0.1745, 0.0215),
    Vec3::from(0.07568, 0.61424, 0.07568),
    Vec3::from(0.633, 0.727811, 0.633),
    0.4,
);

pub const RUBY: Material = Material::new(
    Vec3::from(0.1745, 0.01175, 0.01175),
    Vec3::from(0.61424, 0.04136, 0.04136),
    Vec3::from(0.727811, 0.626959, 0.626959),
    0.6,
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub rougness: f64,
}

impl Material {
    pub const fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, rougness: f64) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            rougness,
        }
    }
}
