use std::vec::IntoIter;

use crate::graphics::triangulation;

use super::{
    structs::{
        Face, ParseError, VertexDataReference, Vertice, VerticeNormal, VerticeParameterSpace,
        VerticeTexture,
    },
    OBJ,
};

pub fn parse_vertice(tokens: &mut IntoIter<&str>, line_n: usize) -> Result<Vertice, ParseError> {
    let x = tokens.next().unwrap().parse::<f32>();
    let y = tokens.next().unwrap().parse::<f32>();
    let z = tokens.next().unwrap().parse::<f32>();
    let w = tokens.next().unwrap_or("1.0").parse::<f32>();

    match (x, y, z, w) {
        (Ok(x), Ok(y), Ok(z), Ok(w)) => Ok(Vertice { x, y, z, w }),
        _ => Err(ParseError::InvalidVertex(line_n, "Error".to_string())),
    }
}

pub fn parse_vertice_parameter_space(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<VerticeParameterSpace, ParseError> {
    let u = tokens.next().unwrap().parse::<f32>();
    let v = tokens.next().unwrap().parse::<f32>();
    let w = tokens.next().unwrap_or("1.0").parse::<f32>();

    match (u, v, w) {
        (Ok(u), Ok(v), Ok(w)) => Ok(VerticeParameterSpace { u, v, w }),
        _ => Err(ParseError::InvalidVertexParameterSpace(
            line_n,
            "Error".to_string(),
        )),
    }
}

pub fn parse_vertice_normal(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<VerticeNormal, ParseError> {
    let i = tokens.next().unwrap().parse::<f32>();
    let j = tokens.next().unwrap().parse::<f32>();
    let k = tokens.next().unwrap().parse::<f32>();

    match (i, j, k) {
        (Ok(i), Ok(j), Ok(k)) => Ok(VerticeNormal { i, j, k }),
        _ => Err(ParseError::InvalidVertexNormal(line_n, "Error".to_string())),
    }
}

pub fn parse_vertice_texture(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<VerticeTexture, ParseError> {
    let u = tokens.next().unwrap().parse::<f32>();
    let v = tokens.next().unwrap_or("0.0").parse::<f32>();
    let w = tokens.next().unwrap_or("0.0").parse::<f32>();

    match (u, v, w) {
        (Ok(u), Ok(v), Ok(w)) => Ok(VerticeTexture { u, v, w }),
        _ => Err(ParseError::InvalidVertexTexture(
            line_n,
            "Error".to_string(),
        )),
    }
}

fn parse_usemtl(line: Option<&str>) -> Option<String> {
    let mut tokens = line?
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .into_iter();

    if tokens.next() == Some("usemtl") {
        Some(tokens.next().unwrap().to_string().clone())
    } else {
        None
    }
}

pub fn parse_face(
    tokens: &mut IntoIter<&str>,
    previous_line: Option<&str>,
    line_n: usize,
) -> Result<Face, ParseError> {
    let material_name = parse_usemtl(previous_line);
    let mut face = Face::partial_new(Vec::new(), material_name);
    let mut is_tripplets_format = false;
    let mut is_twins_format = false;

    for token in tokens {
        let mut parts = token.split("/");

        let v = parts.next().unwrap().parse::<usize>();
        let vt = parts.next().map_or("0", |s| s).parse::<usize>();
        let vn = parts.next().map_or("0", |s| s).parse::<usize>();

        match (v, vt, vn) {
            (Ok(v), Ok(vt), Ok(vn)) => {
                face.vertex_references
                    .push(VertexDataReference::new(v, vt, vn));
                is_tripplets_format = true
            }
            (Ok(v), Err(_), Ok(vn)) => {
                face.vertex_references
                    .push(VertexDataReference::new(v, 0, vn));
                is_twins_format = true
            }
            _ => return Err(ParseError::InvalidFaceSide(line_n, "Error".to_string())),
        }
    }

    if is_tripplets_format && is_twins_format {
        return Err(ParseError::InvalidFace(
            line_n,
            "Illegal to give vertex texture for some vertices, but not all".to_string(),
        ));
    }

    Ok(face)
}

pub fn parse_smoothing_group(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<usize, ParseError> {
    match tokens.next() {
        None => Ok(0_usize),
        Some("off") => Ok(0_usize),
        Some(str) => {
            let id = str.parse::<usize>().map_err(|_| {
                ParseError::InvalidSmoothingGroup(line_n, "Invalid smoothing group".to_string())
            })?;
            Ok(id)
        }
    }
}

pub fn triangulate_polygons(obj: &mut OBJ) {
    for face in obj.faces.iter_mut() {
        let triangulated = triangulation::_2d::fan::triangulate(&mut face.vertex_references);
        face.vertex_references = triangulated;
    }
}
