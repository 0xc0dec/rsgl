#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate winit;
extern crate glm;

mod matrix;
mod vector3;

use vulkano::instance::*;
use vulkano::device::*;
use vulkano::buffer::*;
use vulkano::command_buffer::*;
use vulkano::framebuffer::*;
use vulkano::pipeline::*;
use vulkano::pipeline::viewport::*;
use vulkano::sync::*;
use vulkano::swapchain::*;
use vulkano_win::*;
use winit::*;
use std::sync::Arc;
use vulkano::pipeline::viewport::Viewport;

fn main() {
    let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
        .expect("Failed to create instance.");

    let physical_device = PhysicalDevice::enumerate(&instance).next().expect("No devices available.");
    let queue_family = physical_device.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Failed to find graphics queue.");

    let (device, mut queues) = {
        let extensions = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        Device::new(physical_device, &Features::none(), &extensions,
                    [(queue_family, 0.5)].iter().cloned()
        ).expect("Failed to create device.")
    };

    let queue = queues.next().unwrap();

    // Window
    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .with_title("Test")
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();

    let caps = surface.capabilities(physical_device).expect("Failed to get surface caps.");
    let mut dimensions = caps.current_extent.unwrap_or([1366, 768]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (mut swapchain, mut images) = Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format, dimensions,
                   1, caps.supported_usage_flags, &queue, SurfaceTransform::Identity, alpha,
                   PresentMode::Fifo, true, None)
        .expect("Failed to create swapchain.");

    let vertex_buffer = {
        #[derive(Debug, Clone)]
        struct Vertex {
            position: [f32; 2]
        }
        impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), [
            Vertex { position: [-0.5, -0.25] },
            Vertex { position: [0.0, 0.5] },
            Vertex { position: [0.25, -0.1] }
        ].iter().cloned()).expect("Failed to create vertex buffer.")
    };

    mod vs {
        #[derive(VulkanoShader)]
        #[ty = "vertex"]
        #[src = "
            #version 450

            layout (location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "]
        #[allow(dead_code)]
        struct Dummy;
    }

    mod fs {
        #[derive(VulkanoShader)]
        #[ty = "fragment"]
        #[src = "
            #version 450

            layout (location = 0) out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "]
        #[allow(dead_code)]
        struct Dummy;
    }

    let vs = vs::Shader::load(device.clone()).expect("Failed to create vertex shader module.");
    let fs = fs::Shader::load(device.clone()).expect("Failed to create fragment shader module.");

    let pass = Arc::new(single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());

    let pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer()
        .vertex_shader(vs.main_entry_point(), ())
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap()
    );

    let mut framebuffers: Option<Vec<Arc<vulkano::framebuffer::Framebuffer<_,_>>>> = None;

    let mut recreate_swapchain = false;

    let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;

    let mut dynamic_state = DynamicState {
        line_width: None,
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0 .. 1.0,
        }]),
        scissors: None,
    };

    loop {
        previous_frame_end.cleanup_finished();

        if recreate_swapchain {
            dimensions = surface.capabilities(physical_device)
                .expect("Failed to get surface capabilities")
                .current_extent.unwrap();

            let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => {
                    continue;
                },
                Err(err) => panic!("{:?}", err)
            };

            swapchain = new_swapchain;
            images = new_images;

            framebuffers = None;

            dynamic_state.viewports = Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                depth_range: 0.0 .. 1.0,
            }]);

            recreate_swapchain = false;
        }

        if framebuffers.is_none() {
            framebuffers = Some(images.iter().map(|image| {
                Arc::new(Framebuffer::start(pass.clone())
                    .add(image.clone()).unwrap()
                    .build().unwrap())
            }).collect::<Vec<_>>());
        }

        let (image_num, acquire_future) = match acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                recreate_swapchain = true;
                continue;
            },
            Err(err) => panic!("{:?}", err)
        };

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
            device.clone(), queue.family())
            .unwrap()
            .begin_render_pass(framebuffers.as_ref().unwrap()[image_num].clone(), false,
                               vec![[0.0, 0.0, 1.0, 1.0].into()])
            .unwrap()
            .draw(pipeline.clone(),
                  &dynamic_state,
                  vertex_buffer.clone(), (), ())
            .unwrap()
            .end_render_pass()
            .unwrap()
            .build().unwrap();

        let future = previous_frame_end.join(acquire_future)
            .then_execute(queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                previous_frame_end = Box::new(future) as Box<_>;
            }
            Err(vulkano::sync::FlushError::OutOfDate) => {
                recreate_swapchain = true;
                previous_frame_end = Box::new(vulkano::sync::now(device.clone())) as Box<_>;
            }
            Err(e) => {
                println!("{:?}", e);
                previous_frame_end = Box::new(vulkano::sync::now(device.clone())) as Box<_>;
            }
        }

        let mut run = true;

        events_loop.poll_events(|ev| {
            match ev {
                winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => run = false,
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::MouseInput {
                        button,
                        state: winit::ElementState::Pressed,
                        ..
                    },
                    ..
                } => {
                    println!("Button pressed: {:?}", button);
                },
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::KeyboardInput {
                        input: winit::KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        },
                        ..
                    },
                    ..
                } => {
                    match virtual_keycode {
                        Some(code) => {
                            if code == winit::VirtualKeyCode::Escape {
                                run = false;
                            }

                            println!("Key {:?} {:?}", code, state);
                        },
                        None => ()
                    }
                }
                _ => ()
            }
        });

        if !run { return; }
    }
}
