use glm;

pub trait MatrixExt {
    fn translate(&mut self, v: glm::Vec3);
}

impl MatrixExt for glm::Mat4 {
    fn translate(&mut self, v: glm::Vec3) {
        *self = glm::translate(self, &v)
    }
}
