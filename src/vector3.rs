use glm;

pub trait Vec3Ext {
    fn length(&self) -> f32;
    fn distance(&self, other: glm::Vec3) -> f32;
    fn angle(&self, other: glm::Vec3) -> f32;
    fn normalized(&self) -> Self;
}

impl Vec3Ext for glm::Vec3 {
    fn length(&self) -> f32 {
        glm::length(self)
    }

    fn distance(&self, other: Self) -> f32 {
        glm::distance(self, &other)
    }

    fn angle(&self, other: glm::Vec3) -> f32 {
        glm::angle(self, &other)
    }

    fn normalized(&self) -> Self {
        glm::normalize(self)
    }
}

#[cfg(test)]
mod tests {
    use glm;
    use super::Vec3Ext;

    #[test]
    fn smoke() {
        let v = glm::vec3(1., 2., 3.);
        let l = v.length();
        let d = v.distance(v);
    }
}