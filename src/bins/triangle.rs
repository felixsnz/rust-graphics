use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;
use std::fs;


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(640, 480, "My First Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let program = init_resources().expect("Failed to initialize resources.");

    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        render(program);
    }
    unsafe {
    gl::DeleteProgram(program);
    }

}

// Function to read shader file and return its content as String
fn read_shader_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn compile_shader(shader_type: u32, path: &str) -> Result<u32, String> {
    let source = match read_shader_file(path) {
        Ok(src) => src,
        Err(_) => return Err(String::from("Failed to read shader file")),
    };

    let shader: u32;
    unsafe {
        shader = gl::CreateShader(shader_type);
    }

    let c_str = CString::new(source.as_bytes()).unwrap();

    if shader == 0 {
        return Err(String::from("Failed to create shader"));
    }

    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
    }

    let mut status = gl::FALSE as i32;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    }

    if status == (gl::TRUE as i32) {
        Ok(shader)
    } else {
        Err(String::from("Error compiling shader"))
    }
}


fn init_resources() -> Result<u32, String> {
    let vertex_shader = compile_shader(gl::VERTEX_SHADER, "src/shaders/triangle.vert")?;
    let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, "src/shaders/triangle.frag")?;

    let program = unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        program
    };

    Ok(program)
}


fn render(program: u32) {
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(program);

        let triangle_vertices: [f32; 6] = [0.0, 0.8, -0.8, -0.8, 0.8, -0.8];

        //This variable is just a placeholder for an ID that will refer to the actual VBO stored on the GPU
        let mut vbo = 0;

        //generates a new VBO on the GPU and associates it with an ID
        gl::GenBuffers(1, &mut vbo);
        println!("The value of vbo after gl::GenBuffers is: {}", vbo);

        //binds the VBO so that it will be used for subsequent OpenGL calls targeting gl::ARRAY_BUFFER
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        //Sends data to the GPU vbo
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangle_vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );


        let cstring = CString::new("coord2d").unwrap();
        let coord2d = gl::GetAttribLocation(program, cstring.as_ptr());
        gl::EnableVertexAttribArray(coord2d as u32);
        gl::VertexAttribPointer(coord2d as u32, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        gl::DisableVertexAttribArray(coord2d as u32);
        gl::DeleteBuffers(1, &mut vbo);
    }
}


