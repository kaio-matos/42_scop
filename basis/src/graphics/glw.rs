use std::ffi::CString;
use std::fs;
use std::io;
use std::mem;
use std::os::raw::*;
use std::ptr;

use gl::types;
use gl::types::*;

use crate::math;

///
/// The BufferObject is how we can load stuff into the graphics card memory. The BufferObject can
/// have many types. One example of BufferObject is the Vertex Buffer Object (VBO) which is
/// demonstrated below.
///
///
/// Usage:
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use basis::graphics::glw::BufferObject;
/// use gl;
///
/// let vertices: [f32; 9] = [
///     -0.5, -0.5, 0.0,
///     0.5, -0.5, 0.0,
///     0.0, 0.5, 0.0,
/// ];
/// let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
/// vbo.bind(); // this should be called before any action
/// vbo.store_f32(&vertices);
/// vbo.unbind(); // this should be called after finishing all actions
/// ```
pub struct BufferObject {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferObject {
    ///
    /// Creates a new BufferObject instance with `gl::GenBuffers`.
    ///
    /// * `r#type`
    ///     - `gl::ARRAY_BUFFER`: Vertex information
    ///     - `gl::ELEMENT_ARRAY_BUFFER`: Indice information
    ///     - ...
    /// * `usage`
    ///     - Specify how the graphics card should manage the given data. The available options
    ///     are:
    ///     - `gl::STREAM_DRAW`: the data is set only once and used by the GPU at most a few times.
    ///     - `gl::STATIC_DRAW`: the data is set only once and used many times.
    ///     - `gl::DYNAMIC_DRAW`: the data is changed a lot and used many times.
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id, r#type, usage }
    }

    ///
    /// Tells OpenGL that the current bound context is this buffer,
    /// which means all operations done by the OpenGL functions will affect this buffer.
    ///
    /// This function can be bind multiple buffers at the same time as long as they have different types.
    ///
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    ///
    /// Tells OpenGL that there is no context to operate on.
    ///
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    ///
    /// Copies the passed data into the buffer's memory.
    ///
    pub fn store_f32(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            )
        }
    }

    ///
    /// Store data to the GPU
    ///
    pub fn store_i32(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLint>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            )
        }
    }

    ///
    /// Store data to the GPU
    ///
    pub fn store_u32(&self, data: &[u32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
                &data[0] as *const u32 as *const c_void,
                self.usage,
            )
        }
    }
}

#[derive(Debug)]
pub struct Vao {
    id: gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

///
/// The purpose of the VertexAttribute is to instruct how the shader should interpret the data we
/// are delivering to it, each parameter will be used to help the shader identify what is what
/// during execution. Basically we are saying how it should read the current bound BufferObject.
///
/// Usage:
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///     use std::mem;
///     use std::ptr;
///     use basis::graphics::glw;
///
///     let vao = glw::Vao::new();
///     vao.bind();
///     let position_attribute = glw::VertexAttribute::new(
///     // Which variable we want to setup.
///     // For example considering the following value .glsl code:
///     // `layout(location = 0) in vec3 aPos;`.
///     // The variable that is being set up is the `aPos` variable, because the location is 0.
///     0,
///     // the size of each item
///     3,
///     // the type of the item
///     gl::FLOAT,
///     // ?
///     gl::FALSE,
///     // space between consecutive vertex attributes
///     3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
///     // address pointing to where it should start
///     ptr::null(),
/// );
/// position_attribute.enable();
/// ```
#[derive(Debug)]
pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> Self {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    id: gl::types::GLuint,
}

pub enum ShaderType {
    Vertex(&'static str),
    Fragment(&'static str),
}

impl Shader {
    pub fn new() -> Self {
        let program_id = unsafe {
            let program_id = gl::CreateProgram();
            program_id
        };

        Self { id: program_id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn attach_shader(&self, shader: &ShaderFile) -> &Self {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
        self
    }

    pub fn link(&self) -> Result<(), io::Error> {
        unsafe {
            let mut success: gl::types::GLint = 0;

            gl::LinkProgram(self.id);
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
            if success <= 0 {
                let mut info_log = CString::default();
                let ptr = info_log.into_raw();

                gl::GetProgramInfoLog(self.id, 512, ptr::null_mut(), ptr);
                info_log = CString::from_raw(ptr);

                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    info_log.to_str().unwrap(),
                ));
            }
        }
        Ok(())
    }

    pub fn link_multiple(&self, shaderpaths: Vec<ShaderType>) -> Result<(), io::Error> {
        let shaders = shaderpaths
            .iter()
            .map(|_type| {
                let shaderfile = match _type {
                    ShaderType::Vertex(path) => ShaderFile::new(path, gl::VERTEX_SHADER),
                    ShaderType::Fragment(path) => ShaderFile::new(path, gl::FRAGMENT_SHADER),
                };
                match shaderfile {
                    Ok(ref value) => self.attach_shader(&value),
                    Err(value) => return Err(value),
                };
                shaderfile
            })
            .collect::<Result<Vec<_>, _>>()?;

        unsafe {
            let mut success: gl::types::GLint = 0;

            gl::LinkProgram(self.id);
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
            if success <= 0 {
                let mut info_log = CString::default();
                let ptr = info_log.into_raw();

                gl::GetProgramInfoLog(self.id, 512, ptr::null_mut(), ptr);
                info_log = CString::from_raw(ptr);

                shaders.iter().for_each(|shaderfile| shaderfile.delete());
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    info_log.to_str().unwrap(),
                ));
            }
        }
        shaders.iter().for_each(|shaderfile| shaderfile.delete());
        Ok(())
    }

    pub fn get_uniform_location(&self, name: &'static str) -> UniformLocation {
        let s = CString::new(name).expect("CString::new");
        let id = unsafe { gl::GetUniformLocation(self.id, s.as_ptr()) };

        UniformLocation { id, name }
    }
}

#[derive(Debug)]
pub struct UniformLocation {
    id: gl::types::GLint,
    name: &'static str,
}

impl UniformLocation {
    pub fn uniform1b(&self, v0: bool) {
        unsafe { gl::Uniform1i(self.id, v0 as types::GLint) }
    }

    pub fn uniform1i(&self, v0: types::GLint) {
        unsafe { gl::Uniform1i(self.id, v0) }
    }

    pub fn uniform1f(&self, v0: types::GLfloat) {
        unsafe { gl::Uniform1f(self.id, v0) }
    }

    pub fn uniform3f(&self, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) {
        unsafe { gl::Uniform3f(self.id, v0, v1, v2) }
    }

    pub fn uniform4f(
        &self,
        v0: types::GLfloat,
        v1: types::GLfloat,
        v2: types::GLfloat,
        v3: types::GLfloat,
    ) {
        unsafe { gl::Uniform4f(self.id, v0, v1, v2, v3) }
    }

    pub fn uniform_matrix4fv(&self, mat: &math::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.id, 1, gl::FALSE, mat.as_f32_ptr());
        }
    }
}

#[derive(Debug)]
pub struct ShaderFile {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
}

impl ShaderFile {
    pub fn new(shaderfilepath: &str, r#type: gl::types::GLenum) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(shaderfilepath)?;
        ShaderFile::new_from_source(contents.as_str(), r#type)
    }

    pub fn new_from_source(code: &str, r#type: gl::types::GLenum) -> Result<Self, io::Error> {
        let mut shader_id: u32 = 0;
        unsafe {
            shader_id = gl::CreateShader(r#type);
            let mut success: gl::types::GLint = 0;

            let c_str = CString::new(code).unwrap();
            gl::ShaderSource(shader_id, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success <= 0 {
                let mut info_log = CString::default();
                let ptr = info_log.into_raw();

                gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), ptr);
                info_log = CString::from_raw(ptr);

                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    info_log.to_str().unwrap(),
                ));
            }
        }
        Ok(Self {
            id: shader_id,
            r#type,
        })
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub fn clear_color(
    red: types::GLfloat,
    green: types::GLfloat,
    blue: types::GLfloat,
    alpha: types::GLfloat,
) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
    }
}

pub fn clear(mask: types::GLbitfield) {
    unsafe {
        gl::Clear(mask);
    }
}

pub fn draw_arrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) {
    unsafe {
        gl::DrawArrays(mode, first, count);
    }
}

pub fn draw_elements(
    mode: types::GLenum,
    count: types::GLsizei,
    type_: types::GLenum,
    indices: *const c_void,
) {
    unsafe {
        gl::DrawElements(mode, count, type_, indices);
    }
}

pub fn polygon_mode(face: types::GLenum, mode: types::GLenum) {
    unsafe {
        gl::PolygonMode(face, mode);
    }
}

/// * `pname`
///     - `gl::MAX_VERTEX_ATTRIBS`: The amount of input variables (Vertex Attributes) we can send
///     to a shader
///     - ...
/// * `data` Where the result will be stored
pub fn get_integerv(pname: types::GLenum, data: *mut types::GLint) {
    unsafe {
        gl::GetIntegerv(pname, data);
    }
}
