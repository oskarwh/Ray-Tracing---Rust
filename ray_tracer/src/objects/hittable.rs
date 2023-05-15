/**
 * Public trait for a hittable object
 */
pub trait Hittable 
{
    fn hit(r: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool; 
}
