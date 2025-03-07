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

use structs::{Camera, Rotation};
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
    println!("-------------------------------------------------------------");
    println!("start model_mat: {}", model_mat);
    model_mat.scale(obj.scale);
    model_mat.rotate_around_point(
        obj.center().negate(),
        obj.rotation.radians,
        obj.rotation.axis,
    );
    println!("middle model_mat: {}", model_mat);
    println!("obj.position: {:?}", obj.position);
    model_mat.translate(obj.position);
    println!("final model_mat: {}", model_mat);
    println!("-------------------------------------------------------------");
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

    let positions = [
        math::Vec3::new(0.0, 0.0, 0.0),
        math::Vec3::new(0.0, 0.0, 0.0),
        math::Vec3::new(-1.5, -2.2, -2.5),
        math::Vec3::new(-3.8, -2.0, -12.3),
        math::Vec3::new(2.4, -0.4, -3.5),
        math::Vec3::new(-1.7, 3.0, -7.5),
        math::Vec3::new(1.3, -2.0, -2.5),
        math::Vec3::new(1.5, 2.0, -2.5),
        math::Vec3::new(1.5, 0.2, -1.5),
        math::Vec3::new(-1.3, 1.0, -1.5),
    ];

    let scales = [
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(0.5, 0.5, 0.5),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
        math::Vec3::new(1.0, 1.0, 1.0),
    ];

    let mut rotations = [
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(0.0, 0.0, 0.0),
        ),
        Rotation::new(
            math::Vec3::new(0.0, 0.0, 0.0),
            math::Vec3::new(0.0, 0.0, 0.0),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(1.5, 2.2, 2.5),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(3.8, 2.0, 12.3),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(2.4, 0.4, 3.5),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(1.7, 3.0, 7.5),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(1.3, 2.0, 2.5),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(1.5, 2.0, 2.5),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(1.5, 0.2, 1.5),
        ),
        Rotation::new(
            math::Vec3::new(f32::to_radians(50.), 0.0, 0.0),
            math::Vec3::new(1.3, 1.0, 1.5),
        ),
    ];
    rotations
        .iter_mut()
        .for_each(|r| r.axis = r.axis.normalize());

    for i in 0..1 {
        // for i in 0..positions.len() {
        let mut new = obj.clone();
        let mut rgb = math::Vec3::new(0.5, 0.5, 0.5);
        if i == 0 {
            rgb.x = 1.0;
            rgb.y = 1.0;
        }
        new.color(rgb);
        new.translate(*positions.get(i).unwrap());
        new.rotate(*rotations.get(i).unwrap());
        new.scale(*scales.get(i).unwrap());
        objs.push(new);
    }

    let mut is_wireframe = false;
    let mut camera = Camera::new(
        math::Vec3::new(0.0, 0.0, 3.0),
        math::Vec3::new(0.0, 0.0, -1.0),
        math::Vec3::new(0.0, 1.0, 0.0),
        2.5,
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
            objs[0].rotate(Rotation::new(
                math::Vec3::new(0.1, 0.0, 0.0),
                math::Vec3::new(1.0, 0.0, 0.0),
            ));
        }

        if window.borrow_mut().on_key_hold(
            graphics::glfw::Key::Down,
            graphics::glfw::Modifiers::empty(),
        ) {
            objs[0].rotate(Rotation::new(
                math::Vec3::new(0.1, 0.0, 0.0),
                math::Vec3::new(-1.0, 0.0, 0.0),
            ));
        }
        if window.borrow_mut().on_key_hold(
            graphics::glfw::Key::Left,
            graphics::glfw::Modifiers::empty(),
        ) {
            objs[0].rotate(Rotation::new(
                math::Vec3::new(0.0, 0.1, 0.0),
                math::Vec3::new(0.0, -1.0, 0.0),
            ));
        }
        if window.borrow_mut().on_key_hold(
            graphics::glfw::Key::Right,
            graphics::glfw::Modifiers::empty(),
        ) {
            objs[0].rotate(Rotation::new(
                math::Vec3::new(0.0, 0.1, 0.0),
                math::Vec3::new(0.0, 1.0, 0.0),
            ));
        }

        for obj in objs.iter() {
            draw(&window.borrow_mut(), &obj, &camera);
        }

        window.borrow_mut().update(&mut |_event| {});
    }

    Ok(())
}
