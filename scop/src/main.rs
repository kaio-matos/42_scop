mod helpers;
mod learning_04;
mod learning_05;
mod structs;
mod traits;

use basis::{
    graphics::{
        self,
        glw::{self},
        wavefront,
        window::Window,
    },
    math::{self, VectorFunctions},
};

use structs::Camera;
use traits::Controllable;

use std::cell::RefCell;
use std::rc::Rc;

static WINDOW_HEIGHT: u32 = 800;
static WINDOW_WIDTH: u32 = 800;

fn draw(window: &Window, obj: &structs::Object, camera: &Camera) {
    let shader = glw::Shader::new();
    shader
        .link_multiple(vec![
            glw::ShaderType::Vertex("scop/src/shaders/vertex_perspective_shader.glsl"),
            glw::ShaderType::Fragment("scop/src/shaders/fragment_perspective_shader.glsl"),
        ])
        .unwrap();
    shader.bind();

    let mut model_mat = math::Mat4::identity();
    let mut projection_mat = math::Mat4::perspective(
        45.0_f32.to_radians(),
        (WINDOW_WIDTH / WINDOW_HEIGHT) as f32,
        0.1,
        100.,
    );

    shader
        .get_uniform_location("view")
        .uniform_matrix4fv(&camera.get_view_matrix());
    shader
        .get_uniform_location("projection")
        .uniform_matrix4fv(&projection_mat);

    shader
        .get_uniform_location("color")
        .uniform3f(obj.rgb.x, obj.rgb.y, obj.rgb.z);

    model_mat.scale(obj.scale);
    model_mat.rotate_around_center(obj.center().negate(), obj.rotation);
    model_mat.translate(obj.position);

    shader
        .get_uniform_location("model")
        .uniform_matrix4fv(&model_mat);
    obj.draw();

    shader.unbind();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = Rc::new(RefCell::new(Window::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        "Hello World!",
    )));

    window.borrow_mut().init_gl();

    glw::enable(gl::DEPTH_TEST);

    let obj = structs::Object::new(
        window.clone(),
        wavefront::obj::load("scop/src/resources/cube_colorized_simple/cube_colorized_simple.obj")?,
    );
    let mut objs = Vec::new();

    let objs_transformation = [
        (
            math::Vec3::new(0.0, 0.0, 0.0),            // Position
            math::Vec3::new(1.0, 1.0, 1.0),            // Scale
            math::Quaternion::new(0.0, 0.0, 0.0, 1.0), // Rotation (identity quaternion)
            math::Vec3::new(1.0, 0.0, 0.0),            // Color (Red)
        ),
        (
            math::Vec3::new(5.0, -2.0, 3.0),           // Position
            math::Vec3::new(2.0, 0.5, 1.5),            // Scale
            math::Quaternion::new(0.1, 0.2, 0.3, 0.9), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.0, 1.0, 0.0),            // Color (Green)
        ),
        (
            math::Vec3::new(-4.0, 7.0, 1.0),           // Position
            math::Vec3::new(0.8, 1.0, 1.2),            // Scale
            math::Quaternion::new(0.0, 0.0, 0.0, 1.0), // Rotation (identity quaternion)
            math::Vec3::new(0.0, 0.0, 1.0),            // Color (Blue)
        ),
        (
            math::Vec3::new(1.0, 1.0, 1.0),              // Position
            math::Vec3::new(1.0, 1.0, 1.0),              // Scale
            math::Quaternion::new(0.0, 0.1, 0.0, 0.995), // Rotation (arbitrary quaternion)
            math::Vec3::new(1.0, 1.0, 0.0),              // Color (Yellow)
        ),
        (
            math::Vec3::new(-3.0, -2.0, 5.0),          // Position
            math::Vec3::new(1.2, 0.8, 1.0),            // Scale
            math::Quaternion::new(0.3, 0.4, 0.5, 0.7), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.5, 0.5, 0.5),            // Color (Gray)
        ),
        (
            math::Vec3::new(10.0, 0.0, -3.0),          // Position
            math::Vec3::new(0.5, 0.5, 0.5),            // Scale
            math::Quaternion::new(0.6, 0.0, 0.8, 0.2), // Rotation (arbitrary quaternion)
            math::Vec3::new(1.0, 0.5, 0.5),            // Color (Light Red)
        ),
        (
            math::Vec3::new(2.0, -4.0, 6.0),           // Position
            math::Vec3::new(1.0, 1.0, 2.0),            // Scale
            math::Quaternion::new(0.4, 0.1, 0.6, 0.7), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.5, 0.0, 0.5),            // Color (Purple)
        ),
        (
            math::Vec3::new(0.0, 0.0, -10.0),          // Position
            math::Vec3::new(3.0, 3.0, 3.0),            // Scale
            math::Quaternion::new(0.0, 0.0, 0.0, 1.0), // Rotation (identity quaternion)
            math::Vec3::new(0.5, 0.5, 0.0),            // Color (Olive)
        ),
        (
            math::Vec3::new(-8.0, 5.0, 0.0),           // Position
            math::Vec3::new(1.1, 0.9, 0.8),            // Scale
            math::Quaternion::new(0.1, 0.3, 0.5, 0.8), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.0, 1.0, 1.0),            // Color (Cyan)
        ),
        (
            math::Vec3::new(4.0, -3.0, 2.0),           // Position
            math::Vec3::new(2.0, 2.0, 0.5),            // Scale
            math::Quaternion::new(0.0, 0.0, 0.1, 0.9), // Rotation (arbitrary quaternion)
            math::Vec3::new(1.0, 0.0, 1.0),            // Color (Magenta)
        ),
        (
            math::Vec3::new(-6.0, 2.0, 4.0),           // Position
            math::Vec3::new(0.7, 0.7, 0.7),            // Scale
            math::Quaternion::new(0.5, 0.5, 0.5, 0.5), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.2, 0.6, 0.2),            // Color (Dark Green)
        ),
        (
            math::Vec3::new(8.0, -1.0, -6.0),          // Position
            math::Vec3::new(1.3, 1.3, 1.3),            // Scale
            math::Quaternion::new(0.4, 0.5, 0.6, 0.7), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.9, 0.6, 0.3),            // Color (Orange)
        ),
        (
            math::Vec3::new(-9.0, -4.0, 3.0),          // Position
            math::Vec3::new(1.0, 2.0, 1.0),            // Scale
            math::Quaternion::new(0.3, 0.3, 0.7, 0.8), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.3, 0.3, 0.3),            // Color (Dark Gray)
        ),
        (
            math::Vec3::new(3.0, 2.0, 7.0),            // Position
            math::Vec3::new(1.5, 1.5, 1.5),            // Scale
            math::Quaternion::new(0.2, 0.4, 0.6, 0.8), // Rotation (arbitrary quaternion)
            math::Vec3::new(0.7, 0.7, 0.0),            // Color (Olive Green)
        ),
    ];

    for (position, scale, rotation, rgb) in &objs_transformation {
        let mut new = obj.clone();
        new.scale(*scale);
        new.color(*rgb);
        new.translate(*position);
        new.rotation = *rotation;
        objs.push(new);
    }

    let mut is_wireframe = false;
    let mut camera = Camera::new(
        math::Vec3::new(0.0, 0.0, 50.0),
        math::Vec3::new(0.0, 0.0, -1.0),
        math::Vec3::new(0.0, 1.0, 0.0),
        30.,
        window.clone(),
    );

    while !window.borrow_mut().should_close() {
        window.borrow_mut().compute_deltatime();

        glw::clear_color(0.2, 0.3, 0.3, 1.0);
        glw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        if window
            .borrow_mut()
            .on_key_press(graphics::glfw::Key::E, graphics::glfw::Modifiers::empty())
        {
            is_wireframe = !is_wireframe;
            if is_wireframe {
                glw::polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                glw::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }

        if window
            .borrow_mut()
            .on_key_press(graphics::glfw::Key::Tab, graphics::glfw::Modifiers::empty())
        {
            // TODO:
        }
        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::empty())
        {
            camera.move_up()
        }
        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::empty())
        {
            camera.move_down()
        }
        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::A, graphics::glfw::Modifiers::empty())
        {
            camera.move_left()
        }
        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::D, graphics::glfw::Modifiers::empty())
        {
            camera.move_right()
        }

        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::Control)
        {
            camera.move_forward()
        }

        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::Control)
        {
            camera.move_backward()
        }

        if window
            .borrow_mut()
            .on_key_hold(graphics::glfw::Key::Up, graphics::glfw::Modifiers::empty())
        {
            objs[0]
                .rotation
                .rotate_mut(math::Vec3::new(0.1, 0.0, 0.0), 50_f32.to_radians());
        }

        if window.borrow_mut().on_key_hold(
            graphics::glfw::Key::Down,
            graphics::glfw::Modifiers::empty(),
        ) {
            objs[0]
                .rotation
                .rotate_mut(math::Vec3::new(-0.1, 0.0, 0.0), 50_f32.to_radians());
        }
        if window.borrow_mut().on_key_hold(
            graphics::glfw::Key::Left,
            graphics::glfw::Modifiers::empty(),
        ) {
            objs[0]
                .rotation
                .rotate_mut(math::Vec3::new(0.0, -0.1, 0.0), 50_f32.to_radians());
        }
        if window.borrow_mut().on_key_hold(
            graphics::glfw::Key::Right,
            graphics::glfw::Modifiers::empty(),
        ) {
            objs[0]
                .rotation
                .rotate_mut(math::Vec3::new(0.0, 0.1, 0.0), 50_f32.to_radians());
        }

        for obj in objs.iter_mut() {
            obj.rotation = obj.rotation.normalize();
            draw(&window.borrow_mut(), &obj, &camera);
        }

        window.borrow_mut().update(&mut |_event| {});
    }

    Ok(())
}
