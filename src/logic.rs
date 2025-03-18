use crate::hit::Hit;
use crate::lights::point_light::PointLight;
use crate::math::{vec2::Vec2, vec3::Vec3};
use crate::ray::Ray;
use crate::shapes::shape::Shape;
use crate::{HEIGHT, WIDTH};

const MAX_ITERATIONS: u8 = 2;
const BACKGROUND: Vec3 = Vec3::from(0.3, 0.3, 0.3);

pub fn cast_ray(x: usize, y: usize, shapes: &Vec<Box<dyn Shape>>, light: &PointLight) -> Vec3 {
    let mut uv = Vec2::from(x as f64 / WIDTH as f64, y as f64 / HEIGHT as f64);
    uv = uv * 2.0 - 0.5;
    let aspect = WIDTH as f64 / HEIGHT as f64;
    uv.x *= aspect;
    uv.y *= -1.0;

    let origin = Vec3::from(0.0, 0.0, -2.0);
    let ray = Ray::new(origin, (Vec3::from_vec2(&uv, -1.0) - origin).normalize());

    let hit = hit_check(&ray, shapes, None);

    hit.map(|h| bounce_ray(&h, shapes, light, 0).unwrap_or(get_hit_color(&h, light)))
        .unwrap_or(BACKGROUND)
}

fn bounce_ray(
    hit: &Hit,
    shapes: &Vec<Box<dyn Shape>>,
    light: &PointLight,
    iteration: u8,
) -> Option<Vec3> {
    let reflected = hit.incidence.reflect(&hit.normal);
    let ray = Ray::new(hit.point + f64::MIN_POSITIVE * reflected, reflected);

    let hit = hit_check(&ray, shapes, Some(&hit.shape));

    hit.map(|h| {
        if iteration <= MAX_ITERATIONS {
            bounce_ray(&h, shapes, light, iteration + 1).unwrap_or(get_hit_color(&h, light))
        } else {
            get_hit_color(&h, light)
        }
    })
}

fn hit_is_closer(new_hit: &Hit, old_hit: &Hit) -> bool {
    new_hit.lambda >= f64::MIN_POSITIVE && new_hit.lambda < old_hit.lambda
}

fn get_hit_color(hit: &Hit, light: &PointLight) -> Vec3 {
    let to_light = (light.pos - hit.point).normalize();
    let reflected = hit.incidence.reflect(&hit.normal);
    let falloff = to_light.dot(&reflected);
    let specular = falloff.powf(4.0);

    let mat = &hit.shape.get_material();
    shade(&hit.normal, &to_light) * (mat.ambient + mat.diffuse + specular * mat.specular)
}

fn shade(normal: &Vec3, to_light: &Vec3) -> f64 {
    to_light.dot(normal).max(0.0)
}

fn hit_check(
    ray: &Ray,
    shapes: &Vec<Box<dyn Shape>>,
    exclude: Option<&Box<dyn Shape>>,
) -> Option<Hit> {
    let check = |s: &Box<dyn Shape>, h: &mut Option<Hit>| {
        if let Some(new_hit) = s.hit(&ray) {
            if h.as_ref().map_or(true, |h| hit_is_closer(&new_hit, h)) {
                let _ = h.insert(new_hit);
            }
        }
    };
    exclude.map_or_else(
        || {
            let mut h = None;
            shapes.iter().for_each(|s| check(&s, &mut h));
            h
        },
        |x| {
            let mut h = None;
            shapes
                .iter()
                .filter(|s| *s != x)
                .for_each(|s| check(&s, &mut h));
            h
        },
    )
}
