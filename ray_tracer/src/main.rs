mod vectors;
mod objects;
mod camera;
mod utility;

use vectors::vec3::Point3;

use crate::camera::Camera;
use crate::objects::hittable_list::HittableList;
use crate::objects::sphere::Sphere;
use crate::utility::rtweekend::random_number;
use crate::vectors::vec3::{Color, Vec3};
use crate::vectors::color::*;
use std::rc::Rc;
use std::{io::{self, Write}};

// Image constants
const ASPECT_RATIO: f32 = 16.0/9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 10;
const MAX_DEPTH: i32 = 30;

fn main() 
{
    // World 
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let cam = Camera::default();

    // Render
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let output = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    handle.write_all(output.as_bytes());

    for j in (0..IMAGE_HEIGHT).rev()
    {
        eprintln!("{esc}c", esc = 27 as char);
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH
        {
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..SAMPLES_PER_PIXEL
            {
                let u = ((i as f32) + random_number()) / (IMAGE_WIDTH-1) as f32;
                let v = ((j as f32) + random_number()) / (IMAGE_HEIGHT-1) as f32;
                
                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(&mut handle, &pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.\n");
}