use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::shape::Shape;

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

    fn get_material(&self) -> &Material {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
