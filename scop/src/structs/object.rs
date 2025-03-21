use std::{ffi::c_void, mem, ptr};

use basis::{
    graphics::{
        glw::{self},
        wavefront,
    },
    math::{Quaternion, Vec3, VectorFunctions},
};

#[derive(Debug, Clone)]
pub struct Object {
    pub position: Vec3,
    pub rotation: Quaternion,
    pub scale: Vec3,
    pub rgb: Vec3,

    pub model: wavefront::obj::OBJ,

    vao: glw::Vao,
    texture: glw::Texture,
    cached_center: Vec3,
    cached_vertices: Vec<f32>,
    cached_indices: Vec<u32>,
}

impl Object {
    pub fn new(model: wavefront::obj::OBJ) -> Object {
        let mut object = Object {
            position: Vec3::default(),
            rotation: Quaternion::default(),
            scale: Vec3::default(),
            rgb: Vec3::default(),

            model,
            vao: glw::Vao::new(),
            texture: glw::Texture::new(gl::TEXTURE_3D),
            cached_center: Vec3::default(),
            cached_vertices: Vec::default(),
            cached_indices: Vec::default(),
        };
        object.recompute();
        object.compute_center();
        object
    }

    pub fn color(&mut self, new_color: Vec3) {
        self.rgb = new_color;
    }

    pub fn translate(&mut self, new_pos: Vec3) {
        self.position = new_pos;
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub fn center(&self) -> Vec3 {
        self.cached_center * self.scale // scale by the object's scale
    }

    pub fn draw(&self) {
        self.vao.bind();
        self.texture.active(gl::TEXTURE0);
        self.texture.bind();
        glw::draw_elements(
            gl::TRIANGLES,
            self.cached_indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        self.vao.unbind();
    }

    ///
    /// Refetch all raw vertices and raw indices used by draw call
    /// and setup the VAO and EBO
    ///
    pub fn recompute(&mut self) {
        let model_vertices = self.model.get_raw_vertices();
        let texture = vec![
            0.0, 0.0, // lower-left corner
            1.0, 0.0, // lower-right corner
            0.5, 1.0, // top-center corner
        ];
        let mut vertices = vec![];

        for i in 0..model_vertices.len() / 4 {
            vertices.push(model_vertices[i]);
            vertices.push(model_vertices[i + 1]);
            vertices.push(model_vertices[i + 2]);
            vertices.push(model_vertices[i + 3]);
            vertices.push(self.rgb.x);
            vertices.push(self.rgb.y);
            vertices.push(self.rgb.z);
            if let Some(coord) = texture.get(i) {
                vertices.push(*coord);
            } else {
                vertices.push(0.0);
            }
            if let Some(coord) = texture.get(i + 1) {
                vertices.push(*coord);
            } else {
                vertices.push(0.0);
            }
        }

        self.cached_vertices = vertices;
        self.cached_indices = self.model.get_raw_indices();

        self.vao.bind();
        let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        let ebo = glw::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        vbo.bind();
        ebo.bind();
        vbo.store_f32(&self.cached_vertices);
        ebo.store_u32(&self.cached_indices);

        let start_pointer = ptr::null::<gl::types::GLfloat>();
        let position_attribute = glw::VertexAttribute::new(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            start_pointer as *const c_void,
        );
        position_attribute.enable();
        let color_attribute = glw::VertexAttribute::new(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            unsafe { start_pointer.add(4) as *const c_void },
        );
        color_attribute.enable();
        let texture_coordinate_attribute = glw::VertexAttribute::new(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            unsafe { start_pointer.add(7) as *const c_void },
        );

        texture_coordinate_attribute.enable();

        self.recompute_texture();

        self.vao.unbind();
    }

    fn recompute_texture(&mut self) {
        let border_color = vec![1.0, 1.0, 0.0, 1.0];

        self.texture
            .tex_parameteri(gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
        self.texture
            .tex_parameteri(gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);

        self.texture
            .tex_parameterfv(gl::TEXTURE_BORDER_COLOR, border_color);
        self.texture
            .tex_parameteri(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        self.texture
            .tex_parameteri(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        self.texture.tex_image2d(
            0,
            gl::RGB as i32,
            self.model.texture.0 as i32,
            self.model.texture.1 as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            self.model.texture.2.as_ptr() as *const std::ffi::c_void,
        );

        self.texture.generate_mipmap();
    }

    fn compute_center(&mut self) {
        let mut center = Vec3::default();
        for vertice in &self.model.vertices {
            center = center + Vec3::new(vertice.x, vertice.y, vertice.z);
        }
        self.cached_center = center.scale(1.0 / self.model.vertices.len() as f32);
    }
}
