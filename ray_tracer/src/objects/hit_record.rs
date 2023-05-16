use std::rc::Rc;

use crate::vectors::{vec3::{Point3, Vec3, dot}, ray::Ray};

use super::material::material::Material;

/**
 * A struct holding data about a hit on a Hittable object
 */
#[derive(Copy, Clone)]
pub struct HitRecord<'a>
{
    pub p: Point3, // Point of impact
    pub normal: Vec3, // Normal
    pub mat_ptr: Option<&'a dyn Material>,// Material which ray hit
    pub t: f32, // Root
    pub front_face: bool
}

/**
 * Deafult HitRecord
 */
impl Default for HitRecord<'_>
{
    fn default() -> HitRecord<'static>
    {
        HitRecord { 
            p: Point3::new(0.0,0.0,0.0), 
            normal: Vec3::new(0.0,0.0,0.0), 
            mat_ptr: Option::None,
            t: 0.0, 
            front_face: true 
        }
    }
}

/**
 * Public trait for a HitRecord
 */
impl HitRecord<'_>
{
    // Checks which direction the normal should be pointed, based on the incoming ray.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) 
    {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            // Need to clone as we do not own outward_normal
            outward_normal.clone()
        } else {
            // Do not need to copy as negate_vec() gives us new vec
            outward_normal.negate_vec()
        }
    }
}