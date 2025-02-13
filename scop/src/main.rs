mod learning_04;
mod learning_05;

use basis::graphics;
use basis::math;

use basis::graphics::{
    glw::{self},
    wavefront,
    window::Window,
};

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::atomic::AtomicBool;

static WINDOW_HEIGHT: u32 = 800;
static WINDOW_WIDTH: u32 = 800;

fn draw_square() {
    #[rustfmt::skip]
    let square: [f32; 16] = [
        0.0,   0.0,   0.0, 1.0,
        0.0,   300.0, 0.0, 1.0,
        300.0, 300.0, 0.0, 1.0,
        300.0, 0.0,   0.0, 1.0,
    ];
    let indices: [u32; 6] = [
        0, 1, 2, // first triangle
        0, 3, 2, // second triangle
    ];

    draw_elements(&square, &indices);
}

fn draw_cube() {
    #[rustfmt::skip]
    let cube: [f32; 108] = [
        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

         0.5,  0.5,  0.5,
         0.5,  0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,

        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
    ];

    draw_triangles(&cube);
}

fn draw_triangles(vertices: &[f32]) {
    let vao = glw::Vao::new();
    vao.bind();
    let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32(&vertices);
    let vertex_count = vertices.len() / 3 as usize;
    let position_attribute = glw::VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    vao.unbind();
    vao.bind();
    glw::draw_arrays(gl::TRIANGLES, 0, vertex_count as i32);
    vao.unbind();
}

fn draw_elements(vertices: &[f32], indices: &[u32]) {
    let vao = glw::Vao::new();
    vao.bind();
    let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    let ebo = glw::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    ebo.bind();
    vbo.store_f32(&vertices);
    ebo.store_u32(&indices);
    let data_length: i32 = 4;
    let position_attribute = glw::VertexAttribute::new(
        0,
        data_length,
        gl::FLOAT,
        gl::FALSE,
        data_length * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    glw::draw_elements(
        gl::TRIANGLES,
        indices.len() as i32,
        gl::UNSIGNED_INT,
        // TODO: Investigate why this is working without setting up a EBO
        // indices.as_ptr() as *const c_void,
        ptr::null(),
    );
    vao.unbind();
}

fn draw(window: &Window, obj: &wavefront::obj::OBJ, mat: math::Mat4) {
    let cube_positions = [
        math::Vec3::new(0.0, 0.0, 0.0),
        math::Vec3::new(2.0, 5.0, -15.0),
        math::Vec3::new(-1.5, -2.2, -2.5),
        math::Vec3::new(-3.8, -2.0, -12.3),
        math::Vec3::new(2.4, -0.4, -3.5),
        math::Vec3::new(-1.7, 3.0, -7.5),
        math::Vec3::new(1.3, -2.0, -2.5),
        math::Vec3::new(1.5, 2.0, -2.5),
        math::Vec3::new(1.5, 0.2, -1.5),
        math::Vec3::new(-1.3, 1.0, -1.5),
    ];

    let shader = glw::Shader::new();
    shader
        .link_multiple(vec![
            glw::ShaderType::Vertex("scop/src/shaders/vertex_perspective_shader.glsl"),
            glw::ShaderType::Fragment("scop/src/shaders/fragment_perspective_shader.glsl"),
        ])
        .unwrap();
    shader.bind();
    let mut model_mat = math::Mat4::identity();
    let mut view_mat = math::Mat4::identity();
    let mut projection_mat = math::Mat4::perspective(
        45.0_f32.to_radians(),
        (WINDOW_WIDTH / WINDOW_HEIGHT) as f32,
        0.1,
        100.,
    );

    view_mat.translate(math::Vec3::new(0.0, 0.0, -3.0));
    view_mat.multiply(mat);
    shader
        .get_uniform_location("view")
        .uniform_matrix4fv(&view_mat);
    shader
        .get_uniform_location("projection")
        .uniform_matrix4fv(&projection_mat);

    let time = window.glfw.get_time() as f32;
    let vertices = obj.get_raw_vertices();
    let indices = obj.get_raw_indices();

    for i in 1..100 {
        let random_index = (basis::gen_u32(i) % cube_positions.len() as u32) as usize;
        model_mat.translate(cube_positions[random_index]);
        model_mat.rotate(time * 50_f32.to_radians(), math::Vec3::new(0.0, 0.0, 1.0));

        shader
            .get_uniform_location("model")
            .uniform_matrix4fv(&model_mat);

        draw_elements(&vertices, &indices);
    }

    shader.unbind();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, "Hello World!");

    window.init_gl();

    glw::enable(gl::DEPTH_TEST);

    let mut mat = math::Mat4::identity();
    // let obj = wavefront::obj::load("scop/src/resources/42/42.obj")?;
    let obj =
        wavefront::obj::load("scop/src/resources/cube_colorized_simple/cube_colorized_simple.obj")?;
    let is_wireframe = AtomicBool::new(false);
    let pressed_up = AtomicBool::new(false);
    let pressed_down = AtomicBool::new(false);
    let pressed_left = AtomicBool::new(false);
    let pressed_right = AtomicBool::new(false);
    let pressed_shift_left = AtomicBool::new(false);
    let pressed_shift_right = AtomicBool::new(false);

    while !window.should_close() {
        glw::clear_color(0.2, 0.3, 0.3, 1.0);
        glw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        let pressed_down_v = pressed_down.load(std::sync::atomic::Ordering::Relaxed);
        let pressed_up_v = pressed_up.load(std::sync::atomic::Ordering::Relaxed);
        let pressed_left_v = pressed_left.load(std::sync::atomic::Ordering::Relaxed);
        let pressed_right_v = pressed_right.load(std::sync::atomic::Ordering::Relaxed);
        let pressed_shift_left_v = pressed_shift_left.load(std::sync::atomic::Ordering::Relaxed);
        let pressed_shift_right_v = pressed_shift_right.load(std::sync::atomic::Ordering::Relaxed);

        if pressed_up_v
            || pressed_down_v
            || pressed_left_v
            || pressed_right_v
            || pressed_shift_left_v
            || pressed_shift_right_v
        {
            if pressed_up_v {
                println!("[PRESSING]: Page up");
                mat.translate(math::Vec3::new(0.0, 0.0, 0.1));
            } else if pressed_down_v {
                println!("[PRESSING]: Page down");
                mat.translate(math::Vec3::new(0.0, 0.0, -0.1));
            } else if pressed_left_v {
                println!("[PRESSING]: Left");
                mat.translate(math::Vec3::new(0.1, 0.0, 0.0));
            } else if pressed_right_v {
                println!("[PRESSING]: Right");
                mat.translate(math::Vec3::new(-0.1, 0.0, 0.0));
            } else if pressed_shift_left_v {
                println!("[PRESSING]: Shift Left");
                mat.rotate(-1.0_f32.to_radians(), math::Vec3::new(0.0, 0.0, 1.0));
            } else if pressed_shift_right_v {
                println!("[PRESSING]: Shift Right");
                mat.rotate(1.0_f32.to_radians(), math::Vec3::new(0.0, 0.0, 1.0));
            }

            println!("Mat {}", mat);
        }

        // learning_05::draw_triangles_with_orthographic(&window);
        draw(&window, &obj, mat);

        window.update(&|event| match event {
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::D,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => {
                let value = is_wireframe.load(std::sync::atomic::Ordering::Relaxed);
                if value {
                    glw::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
                } else {
                    glw::polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
                }
                is_wireframe.store(!value, std::sync::atomic::Ordering::Relaxed);
            }

            //
            // Press Shift Left
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Press,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_shift_left.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Release,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_left.store(false, std::sync::atomic::Ordering::Relaxed);
                pressed_shift_left.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            //
            // Press Shift Right
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Press,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_shift_right.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Release,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_right.store(false, std::sync::atomic::Ordering::Relaxed);
                pressed_shift_right.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            //
            // Press Up
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Up,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => {
                pressed_up.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Up,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_up.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            //
            // Press Down
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Down,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => {
                pressed_down.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Down,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_down.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            //
            // Press Left
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => {
                pressed_left.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_left.store(false, std::sync::atomic::Ordering::Relaxed);
                pressed_shift_left.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            //
            // Press Right
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => {
                pressed_right.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_right.store(false, std::sync::atomic::Ordering::Relaxed);
                pressed_shift_right.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            _ => {}
        });
    }

    Ok(())
}
