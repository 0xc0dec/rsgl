pub const VS_TEAPOT: &'static str = "
    #version 140

    in vec3 position;
    in vec3 normal;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 1.0);
    }
";

pub const FS_TEAPOT: &'static str = "
    #version 140

    out vec4 color;
    
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
";