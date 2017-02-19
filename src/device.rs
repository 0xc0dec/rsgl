use glium;
use glium::*;
use glium::glutin::Event;
use glium::glutin::VirtualKeyCode;
use glium::glutin::ElementState;

pub type DeviceDisplay = glium::backend::glutin_backend::GlutinFacade;

pub struct Device {
    running: bool,
    display: DeviceDisplay
}

impl Device {
    pub fn new(title: &str, canvas_width: u32, canvas_height: u32) -> Self {
        let display = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(canvas_width, canvas_height)
            .build_glium()
            .unwrap();

        Device {
            running: true,
            display: display
        }
    }

    pub fn display(&self) -> &DeviceDisplay {
        &self.display
    }

    pub fn run(&mut self, update: &Fn(&mut glium::Frame)) {
        while self.running {
            let events = self.display.poll_events().collect::<Vec<_>>();
            self.process_events(events);

            let mut frame = self.display.draw();
            update(&mut frame);
            frame.finish().unwrap();
        }
    }

    fn process_events(&mut self, evts: Vec<Event>) {
        for e in evts {
            match e {
                Event::Closed => self.stop(),
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => self.stop(),
                _ => ()
            }
        }
    }

    fn stop(&mut self) {
        self.running = false;
    }
} 
