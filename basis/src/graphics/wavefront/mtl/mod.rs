mod helpers;
mod parse_mtl;
mod structs;

use parse_mtl::parse_mtl;
use structs::ParseError;
pub use structs::MTL;

#[derive(Debug)]
pub enum LoadMTLError {
    Io(std::io::Error),
    Parse(ParseError),
}
impl std::error::Error for LoadMTLError {}

impl From<std::io::Error> for LoadMTLError {
    fn from(err: std::io::Error) -> Self {
        LoadMTLError::Io(err)
    }
}

impl From<ParseError> for LoadMTLError {
    fn from(err: ParseError) -> Self {
        LoadMTLError::Parse(err)
    }
}

impl std::fmt::Display for LoadMTLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadMTLError::Io(err) => write!(f, "IO error: {}", err),
            LoadMTLError::Parse(err) => write!(f, "{}", err),
        }
    }
}

pub fn load(file_path: &str) -> Result<MTL, LoadMTLError> {
    let file_content = std::fs::read_to_string(file_path)?;

    let obj = parse_mtl(file_content.as_str())?;

    Ok(obj)
}

pub fn load_files(file_path: Vec<String>) -> Result<Vec<MTL>, LoadMTLError> {
    let mut mtl_files = Vec::new();

    for file in file_path {
        mtl_files.push(load(file.as_str())?);
    }

    Ok(mtl_files)
}
