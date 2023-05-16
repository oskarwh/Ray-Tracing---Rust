use super::{hittable::Hittable, material::material::Material};

pub struct Object
{
    hittable: Box<dyn Hittable>,
    material: Box<dyn Material>
}


impl Object
{
    /**
     * Creates a new object
     */
    pub fn new(hittable: Box<dyn Hittable>, material: Box<dyn Material>) -> Object
    {
        Object{
            hittable: hittable,
            material: material
        }
    }

    /** 
     * Returns a refernce to the hittable object
    */
    pub fn hittable(&self) -> &dyn Hittable
    {
        // Return a refernce of hittable Box content
        &*self.hittable
    }
}