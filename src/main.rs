
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
use std::{f32::INFINITY, char::MAX};
mod camera;
mod material;

use camera::Camera;
use ray::Ray;
mod vec3;
use vec3::Vec3;
use ray::Point3;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use rand::prelude::*;
use material::{Material, scatter};

fn ray_color(ray: &Ray, world: &HittableList, depth: f32) -> Vec3 {


    if depth <= 0.0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

        
        if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {

            let mut scattered = Ray::new(Vec3::default(), Vec3::default());
            let mut attenuation = Vec3::default();

        if scatter(&rec.material, ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1.0);
        }
        
        return Vec3::new(0.0,0.0,0.0);



        // let target = rec.point() + rec.normal() + random_in_unit_sphere();    
        // return ray_color(&Ray::new(rec.point(), target-rec.point()), &world, depth - 1.0) * 0.5;
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0)*(1.0 - t) + Vec3::new(0.5, 0.7, 1.0)*t;
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();
    loop {
        p  = (Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0) - Vec3::new(1.0,1.0,1.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
// fn ray_color(ray:&Ray) -> Colour {
//     let mut t = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, ray);
//     if t > 0.0 {
//         let n = (ray.at(t) - Vec3::new(0.0,0.0,-1.0)).unit_vector();
//         return Colour::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)*0.5;
//     } 
//     let unit_direction = ray.direction().unit_vector();
//     t = (unit_direction.y() + 1.0) * 0.5;
//     return (Colour::new(1.0,1.0,1.0) * (1.0-t)) + Colour::new(0.5, 0.7, 1.0) * t;
// }

// fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
//     let oc: Vec3 = r.origin() - center;
//     let a: f32 = r.direction().length_squared();
//     let half_b: f32 = &oc.dot(r.direction());
//     let c = oc.length_squared() - radius*radius;
//     let discriminant: f32 = half_b*half_b - a*c;
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (-half_b - discriminant.sqrt()) / a;
//     }
// }

fn main() {

    // Image

    const IMAGE_WIDTH: f32 = 1000.0;
    const IMAGE_HEIGHT: f32 = 500.0;
    const SAMPLES_PER_PIXEL: f32 = 100.0;
    const MAX_DEPTH: f32 = 50.0;

    //World 
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)})));
    list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian{albedo: Vec3::new(0.7, 0.3, 0.3)})));
    list.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.8, 0.8), fuzz: 0.3})));
    list.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.6, 0.2), fuzz:1.0})));

    let world = HittableList::new(list);

    // Camera

    let cam = Camera::setup();
    let mut rng = rand::thread_rng();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT as i32).rev() {
        eprintln!("\rScanlines Remaining: {}", j);
        for i in 0..IMAGE_WIDTH as i32 {
            let mut pixel_colour = Vec3::new(0.0, 0.0, 0.0);

            for s in 0..SAMPLES_PER_PIXEL as i32 {
                
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT) as f32;

                let r = &cam.get_ray(u, v);
                pixel_colour += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_colour = pixel_colour / SAMPLES_PER_PIXEL as f32;
            //pixel_colour = Vec3::new(, pixel_colour.y().sqrt(), pixel_colour.z().sqrt());
            let mut r = pixel_colour.x().sqrt();
            let mut g = pixel_colour.y().sqrt();
            let mut b = pixel_colour.z().sqrt();

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.\n");
}

