use glm;
use quaternion::*;

pub struct Transform {
    local_rot: glm::Quat
}

pub enum TransformSpace {
    Own,
    Parent,
    World
}

impl Transform {
    pub fn rotate_by_axis_angle(&mut self, axis: glm::Vec3, angle: f32, space: TransformSpace) {
        let rot = glm::Quat::from_axis_angle(axis, angle);
        self.rotate(rot, space);
    }

    pub fn local_forward(&self) -> glm::Vec3 {
        unimplemented!();
    }

    pub fn local_back(&self) -> glm::Vec3 {
        unimplemented!();
    }

    pub fn local_left(&self) -> glm::Vec3 {
        unimplemented!();
    }

    pub fn local_right(&self) -> glm::Vec3 {
        unimplemented!();
    }

    pub fn local_down(&self) -> glm::Vec3 {
        unimplemented!();
    }

    pub fn local_up(&self) -> glm::Vec3 {
        unimplemented!();
    }

    pub fn translate_local(&self, t: glm::Vec3) {
        unimplemented!();
    }

    fn rotate(&mut self, rot: glm::Quat, space: TransformSpace) {
        match space {
            TransformSpace::Own => {
                self.local_rot = self.local_rot * rot.normalized();
            },
            TransformSpace::Parent => {
                self.local_rot = rot.normalized() * self.local_rot;
            },
            TransformSpace::World => {
                let world_rot = self.world_rotation();
                self.local_rot = self.local_rot * world_rot.inverse() *
                    rot.normalized() * world_rot;
            }
        }
    }

    fn world_rotation(&self) -> glm::Quat {
        unimplemented!();
    }
}