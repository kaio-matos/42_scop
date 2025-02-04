use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(usize, String),
    InvalidValue(usize, String),
}
impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidToken(line, message) => {
                write!(f, "Invalid token at line {}: {}", line, message)
            }
            ParseError::InvalidValue(line, message) => {
                write!(f, "Invalid value at line {}: {}", line, message)
            }
        }
    }
}

/// "r g b" are the values for the red, green, and blue components.
/// The g and b arguments are optional.  If only r is
/// specified, then g, and b are assumed to be equal to r.  The r g b values
/// are normally in the range of 0.0 to 1.0.  Values outside this range
/// increase or decrease the relectivity accordingly.
#[derive(Debug, Default)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Default, PartialEq)]
pub enum IlluminationModel {
    #[default]
    ColorOnAmbientOff,
    ColorOnAmbientOn,
    HighlightOn,
    ReflectionOnRayTraceOn,
    TransparencyGlassOnReflectionRayTraceOn,
    ReflectionFresnelOnRayTraceOn,
    TransparencyRefractionOnReflectionFresnelOffRayTraceOn,
    TransparencyRefractionOnReflectionFresnelOnRayTraceOn,
    ReflectionOnRayTraceOff,
    TransparencyGlassOnReflectionRayTraceOff,
    CastsShadows,
}

impl std::str::FromStr for IlluminationModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(IlluminationModel::ColorOnAmbientOff),
            "1" => Ok(IlluminationModel::ColorOnAmbientOn),
            "2" => Ok(IlluminationModel::HighlightOn),
            "3" => Ok(IlluminationModel::ReflectionOnRayTraceOn),
            "4" => Ok(IlluminationModel::TransparencyGlassOnReflectionRayTraceOn),
            "5" => Ok(IlluminationModel::ReflectionFresnelOnRayTraceOn),
            "6" => Ok(IlluminationModel::TransparencyRefractionOnReflectionFresnelOffRayTraceOn),
            "7" => Ok(IlluminationModel::TransparencyRefractionOnReflectionFresnelOnRayTraceOn),
            "8" => Ok(IlluminationModel::ReflectionOnRayTraceOff),
            "9" => Ok(IlluminationModel::TransparencyGlassOnReflectionRayTraceOff),
            "10" => Ok(IlluminationModel::CastsShadows),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default)]
pub struct DissolveFactor {
    pub factor: f32,
    pub halo: bool,
}

#[derive(Debug, Default)]
pub struct Material {
    pub name: String,
    pub ambient_reflectivity: RGB,
    pub diffuse_reflectivity: RGB,
    pub atmosphere_reflectivity: RGB,
    pub transmission_filter: RGB,
    pub illumination_model: IlluminationModel,
    pub dissolve_factor: DissolveFactor,
    pub specular_highlight_exponent: f32,
    pub sharpness: f32,
    pub optical_density: f32,
}

/*
* Material Library File
*/
#[derive(Debug, Default)]
pub struct MTL {
    pub materials: HashMap<String, Material>,
}
