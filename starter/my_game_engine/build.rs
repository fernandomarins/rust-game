fn main() {
    // Get the include path for GLFW
    let glfw_include_path = "/opt/homebrew/include"; // Adjust this path if needed

    cc::Build::new()
        .file("src/opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("src/opengl_wrapper_lib") // Include your local headers
        .include(glfw_include_path) // Include the GLFW headers
        .compile("opengl_wrapper_lib");
    println!("cargo:rustc-cfg=GL_SILENCE_DEPRECATION");
}