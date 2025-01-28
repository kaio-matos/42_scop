use std::mem;
use std::os::raw::c_void;
use std::ptr;

use basis::graphics::glw;
use basis::graphics::window::Window;
use basis::math;

fn draw_triangles(vertices: &Vec<f32>) {
    let vao = glw::Vao::new();
    vao.bind();
    let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32(&vertices);
    let data_length: i32 = 3;
    let vertex_count = vertices.len() / data_length as usize;
    let position_attribute = glw::VertexAttribute::new(
        0,
        data_length,
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

fn draw_elements(vertices: &Vec<f32>, indices: &Vec<u32>) {
    let vao = glw::Vao::new();
    vao.bind();
    let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32(&vertices);
    let data_length: i32 = 3;
    let vertex_count = vertices.len() / data_length as usize;
    let position_attribute = glw::VertexAttribute::new(
        0,
        data_length,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    vao.unbind();
    vao.bind();
    glw::draw_elements(
        gl::TRIANGLES,
        indices.len() as i32,
        gl::UNSIGNED_INT,
        indices.as_ptr() as *const c_void,
    );
    vao.unbind();
}

pub fn draw_triangles_with_orthographic(window: &Window) {
    let square1: [f32; 18] = [
        300., 300., 0., //
        0.0, 300., 0., //
        0.0, 0.0, 0., //
        //
        0.0, 0.0, 0., //
        300., 300., 0., //
        300., 0.0, 0., //
    ];

    let shader_1 = glw::Shader::new();
    shader_1
        .link_multiple(vec![
            glw::ShaderType::Vertex("scop/src/shaders/learning_05_vertex_shader_orthographic.glsl"),
            glw::ShaderType::Fragment(
                "scop/src/shaders/learning_05_fragment_shader_orthographic.glsl",
            ),
        ])
        .unwrap();

    let mut rotation_mat = math::Mat4::new(math::Vec4::default(1.0));
    let angle = window.glfw.get_time() as f32;
    rotation_mat.scale(math::Vec3::new(0.5, 0.5, 0.5));
    rotation_mat.rotate(angle, math::Vec3::new(0.0, 0.0, 1.0));
    rotation_mat.translate(math::Vec3::new(400., 400., 1.));

    shader_1.bind();
    let transform = shader_1.get_uniform_location("transform");
    transform.uniform_matrix4fv(&rotation_mat);
    let orthographic = shader_1.get_uniform_location("orthographic");
    orthographic.uniform_matrix4fv(&math::Mat4::ortho(0., 800., 0., 800., 0.1, 100.));

    draw_triangles(&square1.to_vec());
    shader_1.unbind();
}

pub fn draw_triangles_with_perspective(window: &Window) {
    let square1: [f32; 20] = [
        0.5, 0.5, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 0.0, 1.0, // top left
    ];
    let indices: [u32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let shader_1 = glw::Shader::new();
    shader_1
        .link_multiple(vec![
            glw::ShaderType::Vertex("scop/src/shaders/learning_05_vertex_shader_perspective.glsl"),
            glw::ShaderType::Fragment(
                "scop/src/shaders/learning_05_fragment_shader_perspective.glsl",
            ),
        ])
        .unwrap();

    let mut model_mat = math::Mat4::identity();
    model_mat.rotate(-55.0_f32.to_radians(), math::Vec3::new(1.0, 0.0, 0.0));

    let mut view_mat = math::Mat4::identity();
    view_mat.translate(math::Vec3::new(0.0, 0.0, -3.0));

    let perspective_mat = math::Mat4::perspective(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);

    shader_1.bind();
    let model = shader_1.get_uniform_location("model");
    let view = shader_1.get_uniform_location("view");
    let projection = shader_1.get_uniform_location("projection");
    model.uniform_matrix4fv(&model_mat);
    view.uniform_matrix4fv(&view_mat);
    projection.uniform_matrix4fv(&perspective_mat);

    draw_elements(&square1.to_vec(), &indices.to_vec());
    shader_1.unbind();
}
