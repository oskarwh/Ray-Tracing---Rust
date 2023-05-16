use crate::{vectors::{ray::Ray, vec3::Color}, objects::hit_record::HitRecord};

/**
 * Implementation for material
 */
pub trait Material
{
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
} 