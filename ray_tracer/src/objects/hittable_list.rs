use std::vec::Vec;
use std::rc::Rc;

use crate::vectors::ray::Ray;

use super::hit_record::HitRecord;
use super::hittable::Hittable;

/**
 * Struct for a list of hittable objects
 */
pub struct HittableList
{
    list: Vec<Rc<dyn Hittable>>
}

impl HittableList
{
    pub fn new() -> HittableList
    {
        HittableList
        {
            list: Vec::<Rc<dyn Hittable>>::new()
        }
    }

    pub fn clear(&mut self)
    {
        self.list.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>)
    {
        self.list.push(object)
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool
    {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.list.clone()
        {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                // How does this work? Why do we change closest hit object on every hit????? The list has to be sorted by distance for this
                closest_so_far = temp_rec.t;
                // Ownership transfers, so we needed to implement copy trait for HitRecord
                // Copy occurs here
                *hit_rec = temp_rec;
            }
        }

        return hit_anything;
    }
}
