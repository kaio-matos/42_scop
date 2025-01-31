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

#[derive(Debug)]
pub struct Vertice {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32, // weight
}
impl Vertice {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vertice {
        Vertice { x, y, z, w }
    }
}
impl PartialEq for Vertice {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FaceSide {
    pub v: usize,
    pub vt: usize,
    pub vn: usize,
}
impl FaceSide {
    pub fn new(v: usize, vt: usize, vn: usize) -> FaceSide {
        FaceSide { v, vt, vn }
    }
}
impl PartialEq for FaceSide {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.vt == other.vt && self.vn == other.vn
    }
}

#[derive(Debug)]
pub struct Face {
    pub sides: Vec<FaceSide>,
    pub material: Option<String>,
}
impl Face {
    pub fn new(sides: Vec<FaceSide>, material: Option<String>) -> Face {
        Face { sides, material }
    }
}
impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.sides == other.sides
    }
}

#[derive(Debug)]
pub struct OBJ {
    //
    // Vertex data
    //
    pub vertices: Vec<Vertice>,
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
    pub mtllib: Option<Vec<String>>,
}

impl OBJ {
    pub fn new() -> OBJ {
        OBJ {
            //
            // Vertex data
            //
            vertices: Vec::new(),
            vertices_texture: Vec::new(),
            vertices_normal: Vec::new(),
            vertices_parameter_space: Vec::new(),

            //
            // Elements
            //
            faces: Vec::new(),

            //
            // Grouping
            //
            name: None,

            //
            // Display/render attributes
            //
            mtllib: None,
        }
    }
}
