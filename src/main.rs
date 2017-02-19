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
mod fs;
mod material;

#[cfg(test)]
mod test;

use glium::*;
use glium::index::PrimitiveType;

fn update(frame: &mut glium::Frame) {
    frame.clear_color(0.0, 0.8, 0.8, 1.0);
}

fn main() {
    let mut dev = device::Device::new("Test", 1366, 768);
    let tex = texture2d::Texture2d::from_bytes(fs::read_all_bytes("assets/water.png"), &dev);
    let mat = material::Material::from_shader_src(shaders::VS_TEAPOT, shaders::FS_TEAPOT, &dev);
    
    dev.run(&update);

    // let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    // let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    // let indices = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

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
