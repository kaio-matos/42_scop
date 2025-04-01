mod helpers;
mod parse_obj;
mod structs;

use structs::ParseError;

use crate::graphics::wavefront;
use crate::graphics::wavefront::obj::parse_obj::parse_obj;

pub use structs::VertexDataReference;
pub use structs::OBJ;

#[derive(Debug)]
pub enum LoadOBJError {
    Io(std::io::Error),
    Parse(ParseError),
    Material(wavefront::mtl::LoadMTLError),
}
impl std::error::Error for LoadOBJError {}

impl From<std::io::Error> for LoadOBJError {
    fn from(err: std::io::Error) -> Self {
        LoadOBJError::Io(err)
    }
}

impl From<ParseError> for LoadOBJError {
    fn from(err: ParseError) -> Self {
        LoadOBJError::Parse(err)
    }
}

impl From<wavefront::mtl::LoadMTLError> for LoadOBJError {
    fn from(err: wavefront::mtl::LoadMTLError) -> Self {
        match err {
            wavefront::mtl::LoadMTLError::Io(err) => {
                LoadOBJError::Material(wavefront::mtl::LoadMTLError::Io(err))
            }
            wavefront::mtl::LoadMTLError::Parse(err) => {
                LoadOBJError::Material(wavefront::mtl::LoadMTLError::Parse(err))
            }
        }
    }
}

impl std::fmt::Display for LoadOBJError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadOBJError::Io(err) => write!(f, "IO error: {}", err),
            LoadOBJError::Parse(err) => write!(f, "{}", err),
            LoadOBJError::Material(err) => write!(f, "{}", err),
        }
    }
}

pub fn load(file_path: &str) -> Result<OBJ, LoadOBJError> {
    let file_content = std::fs::read_to_string(file_path)?;

    let obj = parse_obj(file_content)?;

    // let mtls_paths = obj
    //     .mtls_identifiers
    //     .iter()
    //     .map(|name| {
    //         let mut path = file_path.split('/').collect::<Vec<&str>>();
    //         path.pop();
    //         path.push(name.as_str());
    //         path.join("/")
    //     })
    //     .collect::<Vec<String>>();

    // let mtls = wavefront::mtl::load_files(mtls_paths)?;

    // Ok(obj.load_mtls(mtls).clone())

    Ok(obj.clone())
}
