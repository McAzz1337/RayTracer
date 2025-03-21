use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;

use super::plane::Plane;
use super::sphere::Sphere;
pub trait Shape: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn get_material(&self) -> &Material;
    fn as_any(&self) -> &dyn std::any::Any;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        // Try to downcast both to the same concrete type and compare
        if let (Some(a), Some(b)) = (
            self.as_any().downcast_ref::<Sphere>(),
            other.as_any().downcast_ref::<Sphere>(),
        ) {
            return *a == *b;
        }
        if let (Some(a), Some(b)) = (
            self.as_any().downcast_ref::<Plane>(),
            other.as_any().downcast_ref::<Plane>(),
        ) {
            return a == b;
        }
        false // If they're different types, consider them unequal
    }
}

impl Eq for dyn Shape {}

// impl PartialEq for Box<dyn Shape> {
//     fn eq(&self, other: &Self) -> bool {
//         self.as_ref() == other.as_ref()
//     }
// }
