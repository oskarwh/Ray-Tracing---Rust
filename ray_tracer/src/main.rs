mod vectors;
mod objects;
mod camera;
mod utility;

use objects::material;
use objects::material::material::Material;
use utility::rtweekend::random_number_custom;
use vectors::vec3::{Point3, random_vec, random_vec_custom};

use crate::camera::Camera;
use crate::objects::hittable_list::HittableList;
use crate::objects::material::dielectric::Dielectric;
use crate::objects::material::lambertian::Lambertian;
use crate::objects::material::metal::Metal;
use crate::objects::sphere::Sphere;
use crate::utility::rtweekend::random_number;
use crate::vectors::vec3::{Color, Vec3};
use crate::vectors::color::*;
use std::rc::Rc;
use std::{io::{self, Write}};

// Image constants
const ASPECT_RATIO: f32 = 3.0/2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 70;
const MAX_DEPTH: i32 = 50;

fn main() 
{
    // World 
    let world = random_scene();
    /*
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8,0.8,0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    //let material_left   = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    //let material_center = Rc::new(Dielectric::new(1.5));
    let material_left   = Rc::new(Dielectric::new(1.5));
    let material_right  = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));


    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));*/

    // Camera
    //let cam = Camera::default();
    let lookfrom = Point3::new(13.0,2.0,3.0);
    let lookat = Point3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;//(lookfrom-lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    // Render
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let output = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    handle.write_all(output.as_bytes());

    for j in (0..IMAGE_HEIGHT).rev()
    {
        //eprintln!("{esc}c", esc = 27 as char);
        //eprintln!("\rScanlines remaining: {} ", j);
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


/**
 * Generates image on the cover of the first book
 */
fn random_scene() -> HittableList
{
    let mut world: HittableList = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5,0.5,0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11
    {
        for b in -11..11
        {
            let choose_mat = random_number();
            let center: Point3 = Point3::new(a as f32 + 0.9*random_number(), 0.2, b as f32 + 0.9*random_number());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9
            {
                let material_ground: Rc<dyn Material>;

                if choose_mat < 0.8 
                {
                    // diffuse
                    let albedo = random_vec() * random_vec();
                    let sphere_material  = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if  choose_mat < 0.95
                {
                    // metal
                    let albedo = random_vec_custom(0.5, 1.0);
                    let fuzz = random_number_custom(0.0, 0.5);
                    let sphere_material  = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else 
                {
                    // glass
                    let sphere_material  = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    return world;
}