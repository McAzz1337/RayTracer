use crate::math::vec3::Vec3;

pub struct PointLight {
    pub pos: Vec3,
}

impl PointLight {
    pub fn new(pos: Vec3) -> PointLight {
        PointLight { pos }
    }
}
