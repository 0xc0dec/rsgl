use glm;
use transform::{Transform, TransformSpace};
use math;
use vector3::Vec3Ext;

// TODO As a component/whatever
pub fn update(transform: &mut Transform) {
    let mouse = glm::vec3(1., 2., 3.); // TODO
    let dt = 0.0; // TODO
    let mouse_sensitivity = 0.0; // TODO
    let movement_speed = 0.0; // TODO

    // TODO capture cursor

    if true { // device->isMouseButtonDown(MouseButton::Right, false)
        if !math::equal(mouse.x, 0.) {
            transform.rotate_by_axis_angle(
                glm::vec3(0., 1., 0.),
                mouse_sensitivity * -mouse.x,
                TransformSpace::World
            )
        }

        if !math::equal(mouse.y, 0.) {
            let angle_to_top = transform.local_forward().angle(&glm::vec3(0., 1., 0.));

            let mut delta = mouse_sensitivity * -mouse.y;
            if delta > 0. {
                if angle_to_top - delta <= 0.1 {
                    delta = angle_to_top - 0.1;
                }
            } else {
                if angle_to_top - delta >= 3.04 {
                    delta = angle_to_top - 3.04;
                }
            }

            transform.rotate_by_axis_angle(glm::vec3(1., 0., 0.), delta, TransformSpace::Own);
        }
    }

    let mut movement = glm::vec3(0., 0., 0.); // TODO use new()?
    if true { // device->isKeyPressed(KeyCode::W, false)
        movement += transform.local_forward();
    }
    if true { // device->isKeyPressed(KeyCode::S, false)
        movement += transform.local_back();
    }
    if true { // device->isKeyPressed(KeyCode::A, false)
        movement += transform.local_left();
    }
    if true { // device->isKeyPressed(KeyCode::D, false)
        movement += transform.local_right();
    }
    if true { // device->isKeyPressed(KeyCode::Q, false)
        movement += transform.local_down();
    }
    if true { // device->isKeyPressed(KeyCode::E, false)
        movement += transform.local_up();
    }
    movement = movement.normalized() * dt * movement_speed;

    transform.translate_local(movement);
}