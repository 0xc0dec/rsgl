#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

mod teapot;
mod shaders;

#[cfg(test)]
mod test;

use std::option::*;
use std::io::Cursor;
use glium::*;
use glium::glutin::Event;
use glium::glutin::VirtualKeyCode;
use glium::glutin::ElementState;
use glium::index::PrimitiveType;

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_title("Test!")
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../assets/water.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let program = Program::from_source(&display, shaders::VS_TEAPOT, shaders::FS_TEAPOT, None).unwrap();

    loop {
        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let mut target = display.draw();
        target.clear_color(0.0, 0.8, 0.8, 1.0);
        target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix }, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}
