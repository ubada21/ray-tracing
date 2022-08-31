

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::{PartialEq};

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {

    pub x: f32,
    pub y: f32,
    pub z: f32,

}


impl Vec3 {

    pub fn x(&self) -> f32 {
        return self.x;
    }
    pub fn y(&self) -> f32 {
        return self.y;
    }
    pub fn z(&self) -> f32 {
        return self.z;
    }
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        return self.x < s && self.y < s && self.z < s;
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - (n * ((v.dot(&n)) * 2.0));
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn dot(&self, other: &Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self, other: Self) -> Vec3 {
        return Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x);
        }

    pub fn unit_vector(self) -> Vec3 {
        return self / self.length();
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x : self.x + other.x, y : self.y + other.y, z: self.z + other.z}
    }
}

impl AddAssign for Vec3 {

    fn add_assign(&mut self, other: Self){
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x : self.x - other.x, y : self.y - other.y, z: self.z - other.z}
    }
}

impl SubAssign for Vec3 {

    fn sub_assign(&mut self, other: Self){
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl Mul<f32> for Vec3 {

    type Output = Self;

    fn mul(self, num: f32) -> Self {
        Self {x: self.x * num, y: self.y * num, z: self.z * num}
    }
}


impl MulAssign for Vec3 {

    fn mul_assign(&mut self, other: Self){
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, num: f32) -> Self {
        Self {x: self.x / num, y: self.y / num, z: self.z / num}
    }
}


impl DivAssign for Vec3 {

    fn div_assign(&mut self, other: Self){
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        return &self.x == &other.x  && &self.y == &other.y && &self.z == &other.z
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        
        return Vec3 {x: -self.x, y: -self.y, z: -self.z};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {

        let a = Vec3::new(1.0, 0.0, 2.0);
        let b = Vec3::new(2.0, 3.0, 1.0);
        assert_eq!(a + b, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_add_assign() {

        let mut a = Vec3::new(1.0, 0.0, 2.0);
        let b = Vec3::new(2.0, 3.0, 1.0);
        a += b;
        assert_eq!(a, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(1.0, 0.0, 2.0);
        let b = Vec3::new(2.0, 3.0, 1.0);
        assert_eq!(a - b, Vec3::new(-1.0, -3.0, 1.0));
    }

    #[test]
    fn test_sub_assign() {

        let mut a = Vec3::new(1.0, 0.0, 2.0);
        let b = Vec3::new(2.0, 3.0, 1.0);
        a -= b;
        assert_eq!(a, Vec3::new(-1.0, -3.0, 1.0));
    }

    #[test]
    fn test_mul() {
        let a = Vec3::new(1.0, 2.0, 0.0);
        let b = Vec3::new(2.0, 2.0, 1.0);
        assert_eq!(a * b, Vec3::new(2.0, 4.0, 0.0));
        assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 0.0));
    }

    #[test]
    fn test_mul_assign() {

        let mut a = Vec3::new(1.0, 0.0, 2.0);
        let b = Vec3::new(2.0, 3.0, 2.0);
        a *= b;
        assert_eq!(a, Vec3::new(2.0, 0.0, 4.0));
    }

    #[test]
    fn test_div() {
        let a = Vec3::new(4.0, 16.0, 0.0);
        let b = Vec3::new(2.0, 4.0, 1.0);
        assert_eq!(a / b, Vec3::new(2.0,4.0, 0.0));
        let a = Vec3::new(4.0, 16.0, 0.0);
        assert_eq!(a/2.0, Vec3::new(2.0, 8.0, 0.0));
    }

    #[test]
    fn test_div_assign() {

        let mut a = Vec3::new(2.0, 0.0, 16.0);
        let b = Vec3::new(2.0, 3.0, 4.0);
        a /= b;
        assert_eq!(a, Vec3::new(1.0, 0.0, 4.0));
    }

    #[test]
    fn test_length() {
        let a = Vec3::new(10.0, 2.0, 5.0);
        let b = Vec3::new(4.0, -2.0, 2.0);

        assert_eq!(b.length(), 4.8989795);
        assert_eq!(a.length(), 129_f32.sqrt());
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(10.0, 2.0, 5.0);
        let b = Vec3::new(4.0, -2.0, 2.0);
        let x = a.dot(&b);
        assert_eq!(x, 46.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(10.0, 2.0, 5.0);
        let b = Vec3::new(4.0, -2.0, 2.0);
        let x = a.cross(b);
        assert_eq!(x, Vec3::new(14.0, 0.0, -28.0));
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3::new(10.0, 2.0, 5.0);
        let x = a.unit_vector();
        assert_eq!(x, Vec3::new(0.8804509, 0.17609018, 0.44022545));
    }
}