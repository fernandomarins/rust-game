use my_game_engine::ffi::{initialize_opengl_window, is_key_pressed, check_window_should_close, GLFW_KEY_SPACE};
use my_game_engine::get_window; // Import the get_window function

fn main() {
    // Initialize the OpenGL window with a title and size
    initialize_opengl_window("My Game", 800, 600);
    
    let window: *mut std::ffi::c_void = get_window(); // Get the window pointer
        if window.is_null() {
        eprintln!("Failed to create the window.");
        return;
    }
    if is_key_pressed(window, GLFW_KEY_SPACE) {
        println!("Space bar is pressed!");
    }

    // Main loop (add your game logic here)
    loop {
        // Here you might want to update the window or other game elements

        // Check for other keys or events, and handle them
        // ...

        // Exit condition for the loop (you'll need to implement this)
        if check_window_should_close() { // Call the renamed function
            break; // Exit the loop if the window should close
        }
    }
}