use super::shape::Shape;
use crate::Hit;
use crate::Ray;
use crate::math::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }
}

impl Shape for Plane {
    fn hit(&self, _ray: &Ray) -> Option<Hit> {
        todo!()
    }

    fn get_color(&self) -> Vec3 {
        Vec3::new()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
