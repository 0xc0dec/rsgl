use glm::*;
use glm::ext::*;
use glm;
use vector3;

pub type Matrix = glm::Matrix4<f32>;

pub trait MatrixExt {
    fn translate(&mut self, v: vector3::Vector3);
}

impl MatrixExt for Matrix {
    fn translate(&mut self, v: vector3::Vector3) {
        *self = glm::ext::translate(self, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut _m = Matrix::new();
        let v = vector3::Vector3::new();
        _m.translate(v);
    }
}