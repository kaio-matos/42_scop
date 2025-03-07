use std::{cell::RefCell, mem, ptr, rc::Rc};

use basis::{
    graphics::{glw, wavefront, window::Window},
    math::{Vec3, VectorFunctions},
};

#[derive(Debug, Clone, Default, Copy)]
pub struct Rotation {
    pub radians: Vec3,
    pub axis: Vec3,
}

impl Rotation {
    pub fn new(radians: Vec3, axis: Vec3) -> Self {
        Rotation { radians, axis }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub position: Vec3,
    pub rotation: Rotation,
    pub scale: Vec3,
    pub rgb: Vec3, // TODO: Implement proper material support

    window: Rc<RefCell<Window>>,
    model: wavefront::obj::OBJ,

    cached_center: Vec3,
    cached_vertices: Vec<f32>,
    cached_indices: Vec<u32>,
}

impl Object {
    pub fn new(window: Rc<RefCell<Window>>, model: wavefront::obj::OBJ) -> Object {
        let mut object = Object {
            position: Vec3::default(),
            rotation: Rotation::default(),
            scale: Vec3::default(),
            rgb: Vec3::default(),

            window,
            model,
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

    pub fn rotate(&mut self, rotation: Rotation) {
        self.rotation.radians = self.rotation.radians.add(rotation.radians);
        self.rotation.axis = self.rotation.axis.add(rotation.axis).normalize();
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub fn center(&self) -> Vec3 {
        self.cached_center.mul(self.scale) // scale by the object's scale
    }

    pub fn draw(&self) {
        // TODO: Store vao reference into `self` after the first draw call
        let vao = glw::Vao::new();
        vao.bind();
        let vbo = glw::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        let ebo = glw::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        vbo.bind();
        ebo.bind();
        vbo.store_f32(&self.cached_vertices);
        ebo.store_u32(&self.cached_indices);
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
            self.cached_indices.len() as i32,
            gl::UNSIGNED_INT,
            // TODO: Investigate why this is working without setting up a EBO
            // self.cached_indices.as_ptr() as *const c_void,
            ptr::null(),
        );
        vao.unbind();
    }

    ///
    /// Refetch all raw vertices and raw indices used by draw call
    ///
    pub fn recompute(&mut self) {
        self.cached_vertices = self.model.get_raw_vertices();
        self.cached_indices = self.model.get_raw_indices();
    }

    fn compute_center(&mut self) {
        let mut center = Vec3::default();
        for vertice in &self.model.vertices {
            center = center.add(Vec3::new(vertice.x, vertice.y, vertice.z));
        }
        self.cached_center = center.scale(1.0 / self.model.vertices.len() as f32);
    }
}
