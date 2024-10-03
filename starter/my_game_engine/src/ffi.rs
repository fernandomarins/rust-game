use glfw::ffi::GLFWwindow;

extern "C" {
    fn create_game_window(title: *const i8, width: i32, height: i32);
    fn get_key(window: *mut GLFWwindow, key: i32) -> i32; // Use GLFWwindow from glfw
    fn window_should_close() -> i32; // Add this line to declare window_should_close
}

pub fn initialize_opengl_window(title: &str, width: i32, height: i32) {
    let title_c_string = std::ffi::CString::new(title).expect("CString::new failed");
    
    unsafe {
        create_game_window(title_c_string.as_ptr(), width, height);
    }
}

pub fn is_key_pressed(window: *mut std::ffi::c_void, key: i32) -> bool {
    unsafe {
        get_key(window as *mut GLFWwindow, key) == GLFW_PRESS // Use GLFWwindow here as well
    }
}

pub fn check_window_should_close() -> bool {
    unsafe {
        window_should_close() != 0 // Call the FFI function and return a boolean
    }
}

pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}

pub const GLFW_PRESS: i32 = 1;
pub const GLFW_KEY_SPACE: i32 = 32;
pub const GLFW_KEY_RIGHT: i32 = 262;
pub const GLFW_KEY_LEFT: i32 = 263;
pub const GLFW_KEY_DOWN: i32 = 264;
pub const GLFW_KEY_UP: i32 = 265;
