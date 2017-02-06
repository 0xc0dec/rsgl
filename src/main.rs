#[macro_use]
extern crate glium;
extern crate image;

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

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
            ]
        ).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;

                void main() {
                    v_tex_coords = tex_coords;
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            ",

            fragment: "
                #version 140

                in vec2 v_tex_coords;
                out vec4 color;

                uniform sampler2D tex;

                void main() {
                    color = texture(tex, v_tex_coords);
                }
            "
        }
    ).unwrap();

    loop {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &texture,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.8, 0.8, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
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
