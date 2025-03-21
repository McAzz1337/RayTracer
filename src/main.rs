mod hit;
mod lights;
mod logic;
mod material;
mod math;
mod ray;
mod shapes;

use lights::point_light::PointLight;
use logic::cast_ray;
use material::{CHROME, RUBY};
use math::vec3::Vec3;

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;
use sdl2::video::Window;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas};
use shapes::shape::Shape;
use shapes::sphere::Sphere;
use std::sync::Arc;
use std::thread::{self, Thread};
use std::time::Duration;

const WIDTH: usize = 1440;
const HEIGHT: usize = 1080;

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

    let sphere1 = Sphere::new(Vec3::from(0.75, 0.0, 0.0), 0.5, CHROME);
    let sphere2 = Sphere::new(Vec3::from(-0.75, 0.0, 0.0), 0.5, RUBY);
    let shapes: Arc<Vec<Box<dyn Shape>>> = Arc::new(vec![Box::new(sphere1), Box::new(sphere2)]);

    let light = PointLight::new(Vec3 {
        x: 0.0,
        y: 1.0,
        z: -0.5,
    });

    let mut threads = vec![];
    let num_threads = 7;
    let thread_range = HEIGHT / num_threads;
    let mut i = 0;
    while i < num_threads {
        let light = light.clone();
        let shapes = shapes.clone();
        let t = thread::spawn(move || {
            let start = i * thread_range;
            let end = if i < num_threads {
                (i + 1) * thread_range
            } else {
                WIDTH * HEIGHT - 1
            };
            println!("thread start {} end {}", start * WIDTH, end * WIDTH);
            let mut res = vec![];
            for y in start..end {
                for x in 0..WIDTH {
                    let c = cast_ray(x, y, &shapes, &light);
                    let index = (y * WIDTH + x) * 3;
                    let r = (c.x * 255.0) as u8;
                    let g = (c.y * 255.0) as u8;
                    let b = (c.z * 255.0) as u8;
                    res.push((index, r, g, b))
                }
            }
            res
        });
        threads.push(t);
        i += 1;
    }

    let mut buffer = vec![0u8; WIDTH * HEIGHT * 3];
    for t in threads {
        match t.join() {
            Ok(v) => {
                v.iter().for_each(|x| {
                    let (index, r, g, b) = *x;
                    buffer[index] = r;
                    buffer[index + 1] = g;
                    buffer[index + 2] = b;
                });
            }
            Err(_) => println!("Something went wrong"),
        }
    }

    // for y in 0..HEIGHT as usize {
    //     for x in 0..WIDTH as usize {
    //         let c = cast_ray(x, y, &shapes, &light);
    //         let index = (y * WIDTH + x) * 3;
    //         buffer[index] = (c.x * 255.0) as u8;
    //         buffer[index + 1] = (c.y * 255.0) as u8;
    //         buffer[index + 2] = (c.z * 255.0) as u8;
    //     }
    // }

    println!("Done");
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
