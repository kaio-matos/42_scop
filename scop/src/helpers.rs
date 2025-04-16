use std::{fs, mem, ptr};

use basis::graphics::glw;

#[allow(dead_code)]
pub fn draw_square() {
    #[rustfmt::skip]
    #[allow(unused_variables)]
    let square: [f32; 16] = [
        0.0,   0.0,   0.0, 1.0,
        0.0,   300.0, 0.0, 1.0,
        300.0, 300.0, 0.0, 1.0,
        300.0, 0.0,   0.0, 1.0,
    ];
    #[allow(unused_variables)]
    let indices: [u32; 6] = [
        0, 1, 2, // first triangle
        0, 3, 2, // second triangle
    ];

    // draw_elements(&square, &indices);
}

#[allow(dead_code)]
pub fn draw_cube() {
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

pub fn draw_triangles(vertices: &[f32]) {
    let vao = glw::Vao::new();
    vao.bind();
    let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32(vertices);
    let vertex_count = vertices.len() / 3_usize;
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

pub fn load_custom_texture(path: &str) -> Result<(u32, u32, Vec<u8>), Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;

    let mut lines = file.lines();
    let mut width = 0;
    let mut height = 0;

    // first line `width height`
    if let Some(line) = lines.next() {
        let mut dimensions = line.split_whitespace();
        width = dimensions.next().unwrap_or("").parse::<u32>()?;
        height = dimensions.next().unwrap_or("").parse::<u32>()?;
    }
    let mut texture_data = Vec::new();
    // second line `number number number ....`
    if let Some(line) = lines.next() {
        let data = line.split_whitespace();
        for number in data {
            texture_data.push(number.parse::<u8>()?);
        }
    }

    Result::Ok((width, height, texture_data))
}
