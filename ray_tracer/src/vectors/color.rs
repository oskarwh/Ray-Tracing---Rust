use crate::Color;

use std::{io::{Write, StdoutLock}};

use super::{ray::Ray, vec3::{Point3, dot}};

/**
 * A utility function to write a single pixel's color out to the standard output stream
 */
pub fn write_color(handle: &mut StdoutLock, color: &Color)
{  
    // Write the translated [0,255] value of each color component.
    let x = (255.999 * color.x()) as i32;
    let y = (255.999 * color.y()) as i32;
    let z = (255.999 * color.z()) as i32;

    let output = format!("{} {} {}\n",x, y, z);
    handle.write_all(output.as_bytes());
}

/**
 *  A simple function that returns the color of the background (a simple gradient).
 */
pub fn ray_color(r: &Ray) -> Color
{
    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);

    if t > 0.0
    {
        let n = (r.at(t)- Point3::new(0.0,0.0,-1.0)).unit_vector();
        return Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0).const_mul(0.5);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Color::new(1.0,1.0,1.0).const_mul(1.0-t) + Color::new(0.5,0.7,1.0).const_mul(t)
}

/**
 * Simple function, that hard codes a sphere in the image
 */
pub fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32
{
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - (radius*radius);
    let discriminant = half_b*half_b - a*c;

    // Check if the ray hit the ball
    if discriminant < 0.0
    {
        return -1.0
    } else 
    {
        return (-half_b - discriminant.sqrt()) / a;
    }
}