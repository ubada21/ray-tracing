
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn setup() -> Camera {


        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        }

    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin); 
    }
}