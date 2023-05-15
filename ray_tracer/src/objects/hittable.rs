use crate::vectors::ray::Ray;

use super::hit_record::HitRecord;

/**
 * Public trait for a hittable object
 */
pub trait Hittable 
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool; 
}
