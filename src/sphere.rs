use crate::ray::{Ray, Point3};
use crate::hittable::{HitRecord, Hittable};
use crate::vec3::Vec3;
use crate::material::Material;


pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material, 
}
impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere { center: center, radius: radius, material: material}
    } 

    pub fn center(&self) -> Vec3 {
        return self.center;
    }

    pub fn radius(&self) -> f32 {
        return self.radius;
    }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant: f32 = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }

        
        let sqrtd: f32 = discriminant.sqrt();

        let mut root: f32 = (- half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        return Some(HitRecord {
            t: root,
            point: r.at(root),
            normal: (r.at(root) - self.center) / self.radius,
            material: self.material
        });

    }
}