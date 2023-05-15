/**
 * A struct holding data about a hit on a Hittable object
 */
pub struct HitRecord
{
    p: Point3, // Point of impact
    normal: Vec3, // Normal
    t: f32, // Root
    front_face: bool
}

/**
 * Public trait for a HitRecord
 */
pub trait HitRecord
{
    // Checks which direction the normal should be pointed, based on the incoming ray.
    pub fn set_face_normal(&self, r: &Ray, outward_normal: &Vec3) 
    {
        self.front_face = dot(r.direction(), outward_normal);
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}