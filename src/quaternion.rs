use glm;

pub trait QuatExt {
    fn from_axis_angle(axis: glm::Vec3, angle: f32) -> glm::Quat;
    fn normalized(&self) -> Self;
    fn inverse(&self) -> Self;
}

impl QuatExt for glm::Quat {
    fn from_axis_angle(axis: glm::Vec3, angle: f32) -> glm::Quat {
        glm::quat_angle_axis(angle, &axis)
    }

    fn normalized(&self) -> Self {
        glm::quat_normalize(self)
    }

    fn inverse(&self) -> Self {
        glm::quat_inverse(self)
    }
}

#[cfg(test)]
mod tests {
    use glm;
    use super::QuatExt;

    #[test]
    fn smoke() {
        let q = glm::quat(1., 2., 3., 4.);
        let q = glm::Quat::from_axis_angle(glm::vec3(1., 2., 3.), 1.);
        let q = q.normalized();
        let q = q.inverse();
    }
}