mod learning_04;
mod learning_05;

use basis::graphics::{
    glw::{self},
    window::Window,
};

fn main() {
    let mut window = Window::new(1024, 700, "Hello World!");

    window.init_gl();

    while !window.should_close() {
        glw::clear_color(0.2, 0.3, 0.3, 1.0);
        glw::clear(gl::COLOR_BUFFER_BIT);

        learning_05::draw_triangles_with_perspective(&window);

        window.update();
    }
}
