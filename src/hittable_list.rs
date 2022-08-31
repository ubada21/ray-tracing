use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {

   pub list : Vec<Box<dyn Hittable>>,
}

impl HittableList {

    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {

        HittableList { list }
    }
}


impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far: f32 = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        
        hit_record
    }
}

