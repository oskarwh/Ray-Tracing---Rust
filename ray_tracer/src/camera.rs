use crate::vectors::{vec3::{Point3, Vec3}, ray::Ray};

pub struct Camera
{
    aspect_ratio: f32,
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Default for Camera
{
    /**
     * Creates a camera with the default settings
     */
    fn default() -> Camera
    {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0; 
        let origin = Point3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal.const_div(2.0) - vertical.const_div(2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera{
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner 
        }
    }
}

impl Camera {

    /**
     * Creates ray based on given input (u,v)
     */
    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal.const_mul(u) + self.vertical.const_mul(v) - self.origin)
    }
}