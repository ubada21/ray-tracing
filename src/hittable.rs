use crate::vec3::Vec3;
use crate::ray::{Ray, Point3};
use crate::material::Material;

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        return None;
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

    pub fn set_material(&mut self, val: Material) {
        self.material = val;
    }

    pub fn normal(&self) -> Vec3 {
        return self.normal;
    }
    pub fn material(&self) -> Material {
        return self.material;
    }

    pub fn t(&self) -> f32 {
        return self.t;
    }

    pub fn point(&self) -> Point3 {
        return self.point;
    }

}