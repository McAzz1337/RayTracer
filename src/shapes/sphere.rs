use crate::hit::Hit;
use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::shapes::shape::Shape;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pos: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            pos,
            radius,
            material,
        }
    }
}
impl Shape for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let v = ray.origin - self.pos;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&v);
        let c = v.dot(&v) - self.radius.powf(2.0);

        let d = b.powf(2.0) - 4.0 * a * c;
        let mut hit = None;

        if d >= f64::MIN_POSITIVE {
            let disc = d.sqrt();
            let pos = (-b + disc) / (2.0 * a);
            let neg = (-b - disc) / (2.0 * a);
            let lambda = if neg < f64::MIN_POSITIVE { pos } else { neg };
            if lambda > f64::MIN_POSITIVE {
                let hit_point = ray.origin + lambda * ray.direction;
                let normal = (hit_point - self.pos).normalize();
                hit = Some(Hit::new(
                    Box::new(self.clone()),
                    hit_point,
                    normal,
                    lambda,
                    ray.direction,
                ))
            }
        }

        hit
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
/*
Hit hit_sphere(int idx, Sphere s, Ray ray) {

    vec3 v = ray.origin - s.center;
    float a = dot(ray.dir, ray.dir);
    float b = 2.0 * dot(ray.dir, v);
    float c = dot(v, v) - pow(s.radius, 2.0);

    float d = b * b - 4.0 * a * c;

    if (d >= 0.0) {
        float pos = (-b + sqrt(d)) / (2.0 * a);
        float neg = (-b - sqrt(d)) / (2.0 * a);
        float l = neg < 1e-6 ? pos : neg;
        vec3 q = ray.origin + l * ray.dir;
        vec3 norm = normalize(q - s.center);
        return  Hit(true, idx, q, l, ray.dir, norm, s.color, s.materialIdx, s.gloss);
     }
     return newHit();
}
*/
