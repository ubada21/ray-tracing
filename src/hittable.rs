use crate::vec3::Vec3;
use crate::ray::{Ray, Point3};

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        return false;
    }
}

impl HitRecord {
    
    pub fn set_t(&mut self, val:f32) {
        self.t = val;
    }

    pub fn set_point(&mut self, val:Point3) {
        self.point = val;
    }

    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val;
    }

    pub fn normal(&self) -> Vec3 {
        return self.normal;
    }

    pub fn t(&self) -> f32 {
        return self.t;
    }

    pub fn point(&self) -> Point3 {
        return self.point;
    }

}