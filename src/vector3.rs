use glm;
use glm::*;

pub type Vector3 = glm::Vector3<f32>;

pub trait Vector3Ext {
    fn length(self) -> f32;
    fn distance(self, other: Vector3) -> f32;
}

impl Vector3Ext for Vector3 {
    fn length(self) -> f32 {
        glm::length(self)
    }

    fn distance(self, other: Self) -> f32 {
        glm::distance(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        let _len = v1.length();
        let _dist = v1.distance(v2);
    }
}