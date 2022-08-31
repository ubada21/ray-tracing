use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Copy, Clone)]

pub enum Material {
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3, fuzz: f32},
}

impl Default for Material {
    fn default() -> Material {
       return  Material::Lambertian {albedo: Vec3::new(5.0, 1.0, 1.0)};
    }
}

impl Material {
    
}

pub fn scatter(material: &Material, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match material {
        &Material::Lambertian {albedo} => {
            let scatter_direction = rec.normal() + random_in_unit_sphere() + rec.point();
            *scattered = Ray::new(rec.point(), scatter_direction - rec.point()); 
            *attenuation = albedo;
            return true;
        }

        &Material::Metal {albedo, fuzz} => {
            let reflected = Vec3::reflect(ray_in.direction(), rec.normal());
            *scattered = Ray::new(rec.point(), reflected + random_in_unit_sphere()*fuzz);
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;
        }
    }

}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();
    loop {
        p  = (Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0) - Vec3::new(1.0,1.0,1.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
