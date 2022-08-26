//#![crate_type = "lib"]

use crate::vec3::Vec3;

pub type Point3 = Vec3;
#[derive(Copy, Clone)]
pub struct Ray {

    pub origin: Point3,
    pub direction: Vec3,

}

impl Ray {

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {origin: origin, direction: direction}
    }

    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + self.direction * t;
    }

    pub fn origin(&self) -> Vec3 {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

}
