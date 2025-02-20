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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Controller {
    Element,
    View,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Control {
    controller: Controller,
    element: math::Mat4,
    view: math::Mat4,
}
impl Control {
    fn mat(&self) -> &math::Mat4 {
        match self.controller {
            Controller::Element => &self.element,
            Controller::View => &self.view,
        }
    }

    fn mmat(&mut self) -> &mut math::Mat4 {
        match self.controller {
            Controller::Element => &mut self.element,
            Controller::View => &mut self.view,
        }
    }
}

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

fn draw(window: &Window, obj: &wavefront::obj::OBJ, control: Control) {
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

    view_mat.multiply(control.view);

    shader
        .get_uniform_location("view")
        .uniform_matrix4fv(&view_mat);
    shader
        .get_uniform_location("projection")
        .uniform_matrix4fv(&projection_mat);

    let time = window.glfw.get_time() as f32;
    let vertices = obj.get_raw_vertices();
    let indices = obj.get_raw_indices();

    model_mat.multiply(control.element);

    shader
        .get_uniform_location("model")
        .uniform_matrix4fv(&model_mat);

    draw_elements(&vertices, &indices);

    shader.unbind();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, "Hello World!");

    window.init_gl();

    glw::enable(gl::DEPTH_TEST);

    // let obj = wavefront::obj::load("scop/src/resources/42/42.obj")?;
    // let obj =
    //     wavefront::obj::load("scop/src/resources/cube_colorized_simple/cube_colorized_simple.obj")?;
    let obj = wavefront::obj::load("scop/src/resources/teapot/teapot.obj")?;

    let mut is_wireframe = false;
    let mut pressed_up = false;
    let mut pressed_down = false;
    let mut pressed_left = false;
    let mut pressed_right = false;
    let mut pressed_shift_up = false;
    let mut pressed_shift_down = false;
    let mut pressed_shift_left = false;
    let mut pressed_shift_right = false;
    let mut control = Control {
        controller: Controller::View,
        element: math::Mat4::identity(),
        view: *math::Mat4::identity().translate(math::Vec3::new(0.0, 0.0, -5.0)),
    };

    while !window.should_close() {
        glw::clear_color(0.2, 0.3, 0.3, 1.0);
        glw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        println!("[CONTROLLER] {:?}", control.controller);
        if pressed_up
            || pressed_down
            || pressed_left
            || pressed_right
            || pressed_shift_up
            || pressed_shift_down
            || pressed_shift_left
            || pressed_shift_right
        {
            if pressed_up {
                println!("[PRESSING]: Up");
                control.mmat().translate(math::Vec3::new(0.0, -0.1, 0.0));
            } else if pressed_down {
                println!("[PRESSING]: Down");
                control.mmat().translate(math::Vec3::new(0.0, 0.1, 0.0));
            } else if pressed_left {
                println!("[PRESSING]: Left");
                control.mmat().translate(math::Vec3::new(0.1, 0.0, 0.0));
            } else if pressed_right {
                println!("[PRESSING]: Right");
                control.mmat().translate(math::Vec3::new(-0.1, 0.0, 0.0));
            } else if pressed_shift_up {
                println!("[PRESSING]: Forward");
                control.mmat().translate(math::Vec3::new(0.0, 0.0, 0.1));
            } else if pressed_shift_down {
                println!("[PRESSING]: Backward");
                control.mmat().translate(math::Vec3::new(0.0, 0.0, -0.1));
            } else if pressed_shift_left {
                println!("[PRESSING]: Shift Left (Rotate Left)");
                control
                    .mmat()
                    .rotate(-1.0_f32.to_radians(), math::Vec3::new(0.0, 1.0, 0.0));
            } else if pressed_shift_right {
                println!("[PRESSING]: Shift Right (Rotate Right)");
                control
                    .mmat()
                    .rotate(1.0_f32.to_radians(), math::Vec3::new(0.0, 1.0, 0.0));
            }

            println!("Mat {}", control.mmat());
        }

        // learning_05::draw_triangles_with_orthographic(&window);
        draw(&window, &obj, control);

        window.update(&mut |event| match event {
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::D,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => {
                is_wireframe = !is_wireframe;
                if is_wireframe {
                    glw::polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
                } else {
                    glw::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }

            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Tab,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => match control.controller {
                Controller::Element => {
                    control.controller = Controller::View;
                }
                Controller::View => {
                    control.controller = Controller::Element;
                }
            },

            //
            // Press Shift Up
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Up,
                _,
                graphics::glfw::Action::Press,
                graphics::glfw::Modifiers::Shift,
            ) => pressed_shift_up = true,
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Up,
                _,
                graphics::glfw::Action::Release,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_up = false;
                pressed_shift_up = false;
            }

            //
            // Press Shift Down
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Down,
                _,
                graphics::glfw::Action::Press,
                graphics::glfw::Modifiers::Shift,
            ) => pressed_shift_down = true,
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Down,
                _,
                graphics::glfw::Action::Release,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_down = false;
                pressed_shift_down = false;
            }

            //
            // Press Shift Left
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Press,
                graphics::glfw::Modifiers::Shift,
            ) => pressed_shift_left = true,
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Release,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_left = false;
                pressed_shift_left = false;
            }

            //
            // Press Shift Right
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Press,
                graphics::glfw::Modifiers::Shift,
            ) => pressed_shift_right = true,

            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Release,
                graphics::glfw::Modifiers::Shift,
            ) => {
                pressed_right = false;
                pressed_shift_right = false;
            }

            //
            // Press Up
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Up,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => pressed_up = true,

            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Up,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => pressed_up = false,

            //
            // Press Down
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Down,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => pressed_down = true,

            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Down,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_down = false;
                pressed_shift_down = false;
            }

            //
            // Press Left
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => pressed_left = true,

            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Left,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_left = false;
                pressed_shift_left = false;
            }

            //
            // Press Right
            //
            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Press,
                _,
            ) => pressed_right = true,

            graphics::glfw::WindowEvent::Key(
                graphics::glfw::Key::Right,
                _,
                graphics::glfw::Action::Release,
                _,
            ) => {
                pressed_right = false;
                pressed_shift_right = false;
            }

            _ => {}
        });
    }

    Ok(())
}
