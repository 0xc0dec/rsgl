use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;

#[derive(Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn unit() -> Vector3 {
        Vector3::new(1.0, 1.0, 1.0)
    }

    pub fn unit_x() -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, f: f32) -> Vector3 {
        Vector3 { x: self.x * f, y: self.y * f, z: self.z * f }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, f: f32) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}
