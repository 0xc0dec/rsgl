use device;
use glium::Program;
use glium::uniforms::UniformsStorage;

pub struct Material {
    program: Program
}

impl Material {
    pub fn from_shader_src(vs_src: &str, fs_src: &str, dev: &device::Device) -> Self {
        let program = Program::from_source(dev.display(), vs_src, fs_src, None).unwrap();

        Material {
            program: program
        }
    }

    pub fn set_params(&mut self) {
    	// TODO
    }
}