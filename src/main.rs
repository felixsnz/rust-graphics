use glfw::Context;


fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, _events) = glfw.create_window(640, 480, "My First Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Now you can safely check the OpenGL version
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_str()
            .unwrap();
        println!("OpenGL version: {}", version);
    }

    // ... (rest of your code)
}