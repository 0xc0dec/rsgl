use glm;
use quaternion::*;
use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};

const DIRTY_FLAG_LOCAL: u32 = 1 << 0;
const DIRTY_FLAG_WORLD: u32 = 1 << 1;
const DIRTY_FLAG_INV_TRANSP_WORLD: u32 = 1 << 2;
const DIRTY_FLAG_ALL: u32 = DIRTY_FLAG_LOCAL | DIRTY_FLAG_WORLD | DIRTY_FLAG_INV_TRANSP_WORLD;

#[derive(Debug)]
pub struct Transform {
    local_rot: glm::Quat,
    local_pos: glm::Vec3,
    local_scale: glm::Vec3,
    dirty_flags: Cell<u32>,
    matrix: Cell<glm::Mat4>,
    parent: RefCell<Weak<Transform>>,
    children: RefCell<Vec<Weak<Transform>>>
}

pub enum TransformSpace {
    Own,
    Parent,
    World
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            local_rot: glm::quat_identity(),
            local_pos: glm::vec3(0., 0., 0.),
            local_scale: glm::vec3(1., 1., 1.),
            dirty_flags: Cell::new(DIRTY_FLAG_ALL),
            matrix: Cell::new(glm::identity()),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        }
    }

    pub fn set_parent(&self, parent: Weak<Transform>) {
        if let Some(current) = self.parent.borrow().upgrade() {
            // Remove this item from its previous parent's children list
            println!("{:?}", current);
        }

        *self.parent.borrow_mut() = parent;
    }

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

    fn matrix(&self) -> glm::Mat4 {
        // TODO dirty flags
        glm::scale(
            &glm::translate(
                &glm::quat_cast(&self.local_rot),
                &self.local_pos
            ),
            &self.local_scale
        )
    }

    fn world_matrix(&self) -> glm::Mat4 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let t1 = Rc::new(Transform::new());
        let t2 = Rc::new(Transform::new());
        t1.set_parent(Rc::downgrade(&t2));
    }
}