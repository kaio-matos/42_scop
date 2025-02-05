use std::{iter::Peekable, slice::Iter, vec::IntoIter};

use super::structs::{DissolveFactor, IlluminationModel, Material, ParseError, RGB};

pub fn parse_material(
    name: &str,
    lines: &mut Peekable<Iter<&str>>,
    initial_line_n: usize,
) -> Result<(Material, usize), ParseError> {
    let mut material = Material::default();
    let mut read_lines = 0;

    while let Some(line) = lines.next() {
        let line_n = initial_line_n + read_lines;
        let mut tokens = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .into_iter();

        let statement = tokens.next();

        if statement.is_none() {
            return Err(ParseError::InvalidToken(
                line_n,
                "Missing statement".to_string(),
            ));
        }

        material.name = name.to_string();

        match statement.unwrap() {
            "Ka" => {
                material.ambient_reflectivity = parse_material_rgb(&mut tokens, line_n)?;
            }
            "Kd" => {
                material.diffuse_reflectivity = parse_material_rgb(&mut tokens, line_n)?;
            }
            "Ks" => {
                material.atmosphere_reflectivity = parse_material_rgb(&mut tokens, line_n)?;
            }
            "Tf" => {
                material.transmission_filter = parse_material_rgb(&mut tokens, line_n)?;
            }
            "illum" => {
                material.illumination_model =
                    parse_material_illumination_model(&mut tokens, line_n)?;
            }
            "d" => {
                material.dissolve_factor = parse_material_dissolve_factor(&mut tokens, line_n)?;
            }
            "Ns" => {
                material.specular_highlight_exponent =
                    parse_material_specular_highlight_exponent(&mut tokens, line_n)?;
            }
            "sharpness" => {
                material.sharpness = parse_material_sharpness(&mut tokens, line_n)?;
            }
            "Ni" => {
                material.optical_density = parse_material_optical_density(&mut tokens, line_n)?;
            }
            "#" => {
                continue;
            }
            "newmtl" => {
                break;
            }
            unknown => Err(ParseError::InvalidToken(
                line_n,
                format!("Unknown statement: '{unknown}'"),
            ))?,
        }

        read_lines += 1;
    }

    Ok((material, read_lines))
}

fn parse_material_rgb(tokens: &mut IntoIter<&str>, line_n: usize) -> Result<RGB, ParseError> {
    let r = tokens
        .next()
        .unwrap()
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidToken(line_n, "Invalid R value".to_string()))?;

    let g = tokens
        .next()
        .map_or(r.to_string().as_str(), |s| s)
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidToken(line_n, "Invalid G value".to_string()))?;

    let b = tokens
        .next()
        .map_or(r.to_string().as_str(), |s| s)
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidToken(line_n, "Invalid B value".to_string()))?;

    Ok(RGB { r, g, b })
}

fn parse_material_illumination_model(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<IlluminationModel, ParseError> {
    let model = tokens
        .next()
        .unwrap()
        .parse::<IlluminationModel>()
        .map_err(|_| ParseError::InvalidToken(line_n, "Invalid 'illumn_#' value".to_string()))?;

    Ok(model)
}

fn parse_material_dissolve_factor(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<DissolveFactor, ParseError> {
    let token = tokens.next().unwrap();

    if token == "-halo" {
        let factor = tokens
            .next()
            .unwrap()
            .parse::<f32>()
            .map_err(|_| ParseError::InvalidToken(line_n, "Invalid 'd' value".to_string()))?;

        return Ok(DissolveFactor { factor, halo: true });
    }

    let factor = token
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidToken(line_n, "Invalid 'd' value".to_string()))?;

    Ok(DissolveFactor {
        factor,
        halo: false,
    })
}

fn parse_material_specular_highlight_exponent(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<f32, ParseError> {
    let exponent =
        tokens.next().unwrap().parse::<f32>().map_err(|_| {
            ParseError::InvalidToken(line_n, "Invalid 'exponent' value".to_string())
        })?;

    Ok(exponent)
}

fn parse_material_sharpness(tokens: &mut IntoIter<&str>, line_n: usize) -> Result<f32, ParseError> {
    let sharpness =
        tokens.next().unwrap().parse::<f32>().map_err(|_| {
            ParseError::InvalidToken(line_n, "Invalid 'sharpness' value".to_string())
        })?;

    Ok(sharpness)
}

fn parse_material_optical_density(
    tokens: &mut IntoIter<&str>,
    line_n: usize,
) -> Result<f32, ParseError> {
    let optical_density = tokens
        .next()
        .unwrap()
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidToken(line_n, "Invalid 'Ni' value".to_string()))?;

    if optical_density < 0.001 || optical_density > 10.0 {
        return Err(ParseError::InvalidValue(
            line_n,
            "'Ni' value should range between 0.001 and 10".to_string(),
        ));
    }

    Ok(optical_density)
}
