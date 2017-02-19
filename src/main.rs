#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

mod teapot;
mod shaders;
mod device;
mod texture2d;

#[cfg(test)]
mod test;

use std::option::*;
use glium::*;
use glium::index::PrimitiveType;

fn update() {
}

fn main() {
    let mut dev = device::Device::new("wrewer", 1366, 768);
    let tex = texture2d::Texture2d::from_file("assets/water.png", &dev);
    dev.run(&update);

    // let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    // let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    // let indices = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    // let program = Program::from_source(&display, shaders::VS_TEAPOT, shaders::FS_TEAPOT, None).unwrap();

    // dev.run();
    // loop {
    //     let matrix = [
    //         [0.01, 0.0, 0.0, 0.0],
    //         [0.0, 0.01, 0.0, 0.0],
    //         [0.0, 0.0, 0.01, 0.0],
    //         [0.0, 0.0, 0.0, 1.0f32]
    //     ];

    //     let mut target = display.draw();
    //     target.clear_color(0.0, 0.8, 0.8, 1.0);
    //     target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix }, &Default::default()).unwrap();
    //     target.finish().unwrap();

    //     for ev in display.poll_events() {
    //         match ev {
    //             Event::Closed => return,
    //             Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => return,
    //             _ => ()
    //         }
    //     }
    // }
}
