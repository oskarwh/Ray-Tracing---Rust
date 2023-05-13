mod vectors;

use vectors::vec3::Point3;

use crate::vectors::ray::Ray;
use crate::vectors::vec3::{Color, Vec3};
use crate::vectors::color::*;
use std::{io::{self, Write}};

// Image constants
const ASPECT_RATIO: f32 = 16.0/9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

// Camera constants
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn main() 
{
    // Camera 
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal.const_div(2.0) - vertical.const_div(2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

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
            let u = i as f32 / (IMAGE_WIDTH-1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT-1) as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal.const_mul(u) + vertical.const_mul(v) - origin);
            
            let pixel_color = ray_color(&ray);
            write_color(&mut handle, &pixel_color);
        }
    }
    eprintln!("\nDone.\n");
}