use super::vec3::Point3;
use super::vec3::Vec3;

pub struct Ray 
{
    origin: Point3,
    direction: Vec3
}

impl Default for Ray
{
    fn default() -> Ray {
        Ray {
            origin: Point3::default(),
            direction: Vec3::default()
        }
    }
}

impl Ray
{
    /**
     * Returns a new ray with given start point and direction
     */
    pub fn new(origin: Point3, direction:Vec3) -> Ray
    {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn at(&self, t: f32) -> Point3
    {
        self.origin + self.direction.const_mul(t)
    }

    pub fn origin(&self) -> Vec3
    {
        self.origin
    }

    pub fn direction(&self) -> Point3
    {
        self.direction
    }
}