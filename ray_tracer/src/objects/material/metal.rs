use crate::{vectors::{vec3::{Color, Vec3, random_unit_vector, reflect, dot}, ray::Ray}, objects::hit_record::HitRecord};

use super::material::Material;

pub struct Metal
{
    pub albedo: Color
}

impl Metal
{
    pub fn new(a: Color) -> Metal
    {
        Metal
        {
            albedo: a
        }
    }
}

impl Material for Metal
{
    fn scatter(&self, 
        r_in: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Color, 
        scattered: &mut Ray) -> bool 
    {
        let reflected: Vec3 = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        // Return
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}