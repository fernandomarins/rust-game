pub mod ffi;

use std::ffi::c_void;

use glfw::{Action, Context, Key, WindowHint};
extern crate gl;
extern crate image;

static mut WINDOW_PTR: *mut glfw::ffi::GLFWwindow = std::ptr::null_mut(); 

pub struct GameWindow {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
}

impl GameWindow {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)); // Correct usage

        let (mut window, _events) = glfw.create_window(
            width as u32, 
            height as u32, 
            title, 
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

        // Store the raw pointer to the GLFW window in the global variable
        unsafe {
            WINDOW_PTR = window.window_ptr() as *mut glfw::ffi::GLFWwindow; // Use window_ptr() method
        }

        window.make_current(); // Make the window's context current
        window.set_key_polling(true); // Enable key polling

        GameWindow { glfw, window }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.get_key(key) == Action::Press
    }

    pub fn clear_screen(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

pub fn create_window(width: i32, height: i32) -> GameWindow {
    GameWindow::new("My Game", width, height)
}

pub fn window_should_close(window: &GameWindow) -> bool {
    window.should_close()
}

pub fn update_game_window(window: &mut GameWindow) {
    window.swap_buffers();
}

pub fn create_sprite(path: &str) -> u32 {
    let img = image::open(path).unwrap().to_rgba8();
    let (width, height) = img.dimensions();

    let mut texture_id: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_raw().as_ptr() as *const std::ffi::c_void,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    texture_id
}

pub fn render_sprite(sprite_id: u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, sprite_id);
        // Add your rendering code here.
    }
}

pub fn clear_screen(window: &GameWindow) {
    window.clear_screen();
}

pub fn get_window() -> *mut c_void {
    unsafe { WINDOW_PTR as *mut c_void } // Return the stored window pointer
}

pub fn get_key(key: &str, window: &GameWindow) -> bool {
    match key {
        "UP" => window.is_key_pressed(Key::Up),
        "DOWN" => window.is_key_pressed(Key::Down),
        "LEFT" => window.is_key_pressed(Key::Left),
        "RIGHT" => window.is_key_pressed(Key::Right),
        "SPACE" => window.is_key_pressed(Key::Space),
        _ => false,
    }
}

// fn main() {
//     let window = create_window(800, 600);

//     loop {
//         if window_should_close(&window) {
//             println!("Window is closing!");
//             break;
//         }

//         if window.is_key_pressed(Key::Space) {
//             println!("Space bar is pressed!");
//         }

//         clear_screen(&window);

//         std::thread::sleep(std::time::Duration::from_millis(16)); // roughly 60 FPS
//     }
// }
