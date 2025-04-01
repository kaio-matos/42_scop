use std::collections::HashMap;

use crate::{
    graphics::{
        aabb::AABB,
        wavefront::{
            self,
            mtl::{Material, MTL},
        },
    },
    math,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(usize, String),
    InvalidValue(usize, String),
    InvalidVertex(usize, String),
    InvalidVertexTexture(usize, String),
    InvalidVertexNormal(usize, String),
    InvalidVertexParameterSpace(usize, String),
    InvalidFace(usize, String),
    InvalidFaceSide(usize, String),
    InvalidFaceMaterial(usize, String),
    InvalidGroup(usize, String),
    InvalidSmoothingGroup(usize, String),
    InvalidMaterialLibrary(usize, String),
}
impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidToken(line, token) => {
                write!(f, "Invalid token at line {}: {}", line, token)
            }
            ParseError::InvalidValue(line, value) => {
                write!(f, "Invalid value at line {}: {}", line, value)
            }
            ParseError::InvalidFace(line, face) => {
                write!(f, "Invalid face at line {}: {}", line, face)
            }
            ParseError::InvalidVertex(line, vertex) => {
                write!(f, "Invalid vertex at line {}: {}", line, vertex)
            }
            ParseError::InvalidVertexTexture(line, vertex_texture) => {
                write!(
                    f,
                    "Invalid vertex texture at line {}: {}",
                    line, vertex_texture
                )
            }
            ParseError::InvalidVertexNormal(line, vertex_normal) => {
                write!(
                    f,
                    "Invalid vertex normal at line {}: {}",
                    line, vertex_normal
                )
            }
            ParseError::InvalidVertexParameterSpace(line, vertex_parameter_space) => {
                write!(
                    f,
                    "Invalid vertex parameter space at line {}: {}",
                    line, vertex_parameter_space
                )
            }
            ParseError::InvalidFaceSide(line, face_side) => {
                write!(f, "Invalid face side at line {}: {}", line, face_side)
            }
            ParseError::InvalidFaceMaterial(line, face_material) => {
                write!(
                    f,
                    "Invalid face material at line {}: {}",
                    line, face_material
                )
            }
            ParseError::InvalidGroup(line, group) => {
                write!(f, "Invalid group at line {}: {}", line, group)
            }
            ParseError::InvalidSmoothingGroup(line, group) => {
                write!(f, "Invalid smoothing group at line {}: {}", line, group)
            }
            ParseError::InvalidMaterialLibrary(line, material_library) => {
                write!(
                    f,
                    "Invalid material library at line {}: {}",
                    line, material_library
                )
            }
        }
    }
}

///////////////////////////////////
// Vertex data
///////////////////////////////////

#[derive(Debug, Clone, Default)]
pub struct VerticeParameterSpace {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}
impl VerticeParameterSpace {
    pub fn new(u: f32, v: f32, w: f32) -> VerticeParameterSpace {
        VerticeParameterSpace { u, v, w }
    }
}
impl PartialEq for VerticeParameterSpace {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v && self.w == other.w
    }
}

#[derive(Debug, Clone, Default)]
pub struct VerticeNormal {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}
impl VerticeNormal {
    pub fn new(i: f32, j: f32, k: f32) -> VerticeNormal {
        VerticeNormal { i, j, k }
    }
}
impl PartialEq for VerticeNormal {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.k == other.k
    }
}

#[derive(Debug, Clone, Default)]
pub struct VerticeTexture {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}
impl VerticeTexture {
    pub fn new(u: f32, v: f32, w: f32) -> VerticeTexture {
        VerticeTexture { u, v, w }
    }
}
impl PartialEq for VerticeTexture {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v && self.w == other.w
    }
}

///////////////////////////////////
// Elements
///////////////////////////////////

#[derive(Debug, Clone, Default)]
pub struct SmoothingGroup<'a> {
    pub id: usize,
    pub faces: Vec<&'a Face>,
}

#[derive(Debug, Clone, Default)]
pub struct VertexDataReference {
    pub v: usize,
    pub vt: usize,
    pub vn: usize,
}
impl VertexDataReference {
    pub fn new(v: usize, vt: usize, vn: usize) -> VertexDataReference {
        VertexDataReference { v, vt, vn }
    }
}
impl PartialEq for VertexDataReference {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.vt == other.vt && self.vn == other.vn
    }
}

#[derive(Debug, Clone, Default)]
pub struct Face {
    pub vertex_references: Vec<VertexDataReference>,
    pub material_name: Option<String>,
    pub material: Option<Material>,
    pub smoothing_group: Option<usize>,
}
impl Face {
    pub fn partial_new(
        vertex_references: Vec<VertexDataReference>,
        material_name: Option<String>,
    ) -> Face {
        Face {
            vertex_references,
            material_name,
            material: None,
            smoothing_group: None,
        }
    }

    pub fn is_partial(&self) -> bool {
        self.material.is_none()
    }

    pub fn set_material(&mut self, material: Option<Material>) {
        self.material = material;
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.vertex_references == other.vertex_references
    }
}

#[derive(Debug, Clone, Default)]
pub struct OBJ {
    //
    // Vertex data
    //
    pub vertices: Vec<math::Vec4>,
    pub vertices_texture: Vec<VerticeTexture>,
    pub vertices_normal: Vec<VerticeNormal>,
    pub vertices_parameter_space: Vec<VerticeParameterSpace>,

    //
    // Elements
    //
    pub faces: Vec<Face>,

    //
    // Grouping
    //
    pub name: Option<String>,

    //
    // Display/render attributes
    //
    pub mtls_identifiers: Vec<String>,
    pub mtls: Vec<wavefront::mtl::MTL>,

    //
    // Texture
    //
    pub texture: (u32, u32, Vec<u8>),
}

impl OBJ {
    pub fn has_loaded_materials(&self) -> bool {
        self.mtls.len() > 0
    }

    pub fn get_raw_vertices(&self, rgb: math::Vec3) -> Vec<f32> {
        let mut vertices_texture = self.vertices_texture.clone();

        let aabb = AABB::from(&self.vertices);

        // if there is no texture vertices we calculate ourselves based on the aabb
        if vertices_texture.is_empty() {
            for vertice in &self.vertices {
                let mut range = math::Vec3::default();
                range.x = aabb.max.x - aabb.min.x;
                range.y = aabb.max.y - aabb.min.y;
                range.z = aabb.max.z - aabb.min.z;
                if range.x == 0.0 {
                    range.x = 1.0;
                }
                if range.y == 0.0 {
                    range.y = 1.0;
                }
                if range.z == 0.0 {
                    range.z = 1.0;
                }

                vertices_texture.push(VerticeTexture::new(
                    (vertice.x - aabb.min.x) / range.x,
                    (vertice.y - aabb.min.y) / range.y,
                    (vertice.z - aabb.min.z) / range.z,
                ));
            }
        }

        self.faces.iter().fold(
            Vec::with_capacity(self.faces.len() * 10),
            |mut acc, face| {
                for reference in &face.vertex_references {
                    let rvt = if self.vertices_texture.is_empty() {
                        // if the texture was generated, we use the vertice reference
                        reference.v - 1
                    } else {
                        // if the texture is from the OBJ file, we use their references
                        reference.vt - 1
                    };

                    let v = if let Some(vertice) = self.vertices.get(reference.v - 1) {
                        vertice
                    } else {
                        &math::Vec4::default()
                    };
                    let vt = if let Some(texture) = vertices_texture.get(rvt) {
                        texture
                    } else {
                        &VerticeTexture::default()
                    };

                    acc.push(v.x);
                    acc.push(v.y);
                    acc.push(v.z);
                    acc.push(v.w);
                    acc.push(rgb.x);
                    acc.push(rgb.y);
                    acc.push(rgb.z);
                    acc.push(vt.u);
                    acc.push(vt.v);
                    acc.push(vt.w);
                }

                acc
            },
        )
    }

    pub fn get_raw_indices(&self) -> Vec<u32> {
        self.faces.iter().fold(Vec::new(), |mut acc, face| {
            face.vertex_references.iter().for_each(|reference| {
                acc.push((reference.v - 1) as u32);
            });
            acc
        })
    }

    pub fn load_mtls(&mut self, mtls: Vec<MTL>) -> &Self {
        self.mtls = mtls;

        for face in self.faces.iter_mut() {
            if face.material_name.is_none() {
                continue;
            }
            if let Some(name) = face.material_name.clone() {
                for material in self.mtls.iter() {
                    face.set_material(material.get(&name).cloned());
                }
            }
        }
        self
    }

    pub fn get_smoothing_group_by_id(&self, id: usize) -> SmoothingGroup {
        let faces = self
            .faces
            .iter()
            .filter(|face| face.smoothing_group == Some(id))
            .collect::<Vec<&Face>>();

        SmoothingGroup { id, faces }
    }
}
