use crate::shapes::shape::Shape;

use crate::Vec3;

pub struct Hit {
    pub shape: Box<dyn Shape>,
    pub point: Vec3,
    pub normal: Vec3,
    pub lambda: f64,
    pub incidence: Vec3,
}

impl Hit {
    pub fn new(
        shape: Box<dyn Shape>,
        point: Vec3,
        normal: Vec3,
        lambda: f64,
        incidence: Vec3,
    ) -> Hit {
        Hit {
            shape,
            point,
            normal,
            lambda,
            incidence,
        }
    }
}
