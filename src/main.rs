mod colors;
mod hit;
mod lights;
mod math;
mod shapes;

use colors::*;
use hit::Hit;
use lights::point_light::PointLight;
use math::{ray::Ray, vec2::Vec2, vec3::Vec3};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;
use sdl2::video::Window;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas};
use shapes::shape::Shape;
use shapes::sphere::Sphere;
use std::time::Duration;

const WIDTH: usize = 1440;
const HEIGHT: usize = 1080;

const MAX_ITERATIONS: u8 = 2;

fn hit_is_closer(new_hit: &Hit, old_hit: &Hit) -> bool {
    new_hit.lambda >= f64::MIN_POSITIVE && new_hit.lambda < old_hit.lambda
}

fn get_hit_color(hit: &Hit, light: &PointLight) -> Vec3 {
    let color = hit.shape.get_color();
    let to_light = light.pos - hit.point;
    let phi = to_light.dot(&hit.normal).max(0.0);
    phi * color
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

fn cast_ray(x: usize, y: usize, shapes: &Vec<Box<dyn Shape>>, light: &PointLight) -> Vec3 {
    let mut uv = Vec2::from(x as f64 / WIDTH as f64, y as f64 / HEIGHT as f64);
    uv = uv * 2.0 - 0.5;
    let aspect = WIDTH as f64 / HEIGHT as f64;
    uv.x *= aspect;
    uv.y *= -1.0;

    let origin = Vec3::from(0.0, 0.0, -2.0);
    let ray = Ray::new(origin, (Vec3::from_vec2(&uv, -1.0) - origin).normalize());

    let hit = hit_check(&ray, shapes, None);

    hit.map(|h| bounce_ray(&h, shapes, light, 0).unwrap_or(get_hit_color(&h, light)))
        .unwrap_or(BG)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Raytracer", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Streaming, 800, 600)
        .expect("");

    let sphere1 = Sphere::new(Vec3::from(0.75, 0.0, 0.0), 0.5, RED);
    let sphere2 = Sphere::new(Vec3::from(-0.75, 0.0, 0.0), 0.5, BLUE);
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(sphere1), Box::new(sphere2)];

    let light = PointLight::new(Vec3 {
        x: 0.0,
        y: 1.0,
        z: -0.5,
    });

    let mut buffer = vec![0u8; WIDTH * HEIGHT * 3];
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let c = cast_ray(x, y, &shapes, &light);
            let index = (y * WIDTH + x) * 3;
            buffer[index] = (c.x * 255.0) as u8;
            buffer[index + 1] = (c.y * 255.0) as u8;
            buffer[index + 2] = (c.z * 255.0) as u8;
        }
    }

    texture.update(None, &buffer, WIDTH * 3).expect("");
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        // Limit FPS
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
