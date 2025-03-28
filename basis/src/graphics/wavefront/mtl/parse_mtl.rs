use super::helpers;
use super::structs::{ParseError, MTL};

pub trait Advance {
    fn advance(&mut self, n: usize) -> &Self;
}

impl<'a> Advance for std::slice::Iter<'a, &str> {
    fn advance(&mut self, n: usize) -> &Self {
        let mut n = n;

        while (n > 0) && self.next().is_some() {
            n -= 1;
        }
        self
    }
}

pub fn parse_mtl(data: &str) -> Result<MTL, ParseError> {
    let mut mtl = MTL::default();

    let binding = data
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let mut lines = binding.iter();
    let mut current_line: usize = 1;

    while let Some(line) = lines.next() {
        let mut tokens = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .into_iter();
        let command = tokens.next();

        if command.is_none() {
            return Err(ParseError::InvalidToken(
                current_line,
                "Missing command".to_string(),
            ));
        }

        match command.unwrap() {
            "newmtl" => {
                match tokens.next() {
                    Some(name) => {
                        let (material, read) = helpers::parse_material(
                            name,
                            &mut lines.clone().peekable(),
                            current_line,
                        )?;
                        mtl.insert(name.to_string(), material);
                        lines.advance(read);
                    }
                    None => {
                        return Err(ParseError::InvalidToken(
                            current_line,
                            "Missing material name".to_string(),
                        ));
                    }
                }
                Ok(())
            }
            "#" => {
                // Ignore comments
                Ok(())
            }
            unknown => {
                // Err(ParseError::InvalidToken(
                //     current_line,
                //     format!("Unknown token: '{unknown}'"),
                // ))
                Ok(())
            }
        }?;
        current_line += 1;
    }

    Ok(mtl)
}

#[cfg(test)]
mod tests {
    use crate::graphics::wavefront::mtl::structs::IlluminationModel;

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn it_should_ignore_comments() {
        let file = "
            newmtl Rock
            # Ka 0.2 0.2 0.2
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(rock.ambient_reflectivity.r, 0.0);
        assert_eq!(rock.ambient_reflectivity.r, 0.0);
        assert_eq!(rock.ambient_reflectivity.r, 0.0);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_store_name() {
        let file = "
            newmtl Rock
            Ka 0.2 0.2 0.2
";

        let result = parse_mtl(file).expect("This should work");

        assert_eq!(result.len(), 1);
        assert!(result.get("Rock").is_some());
        assert_eq!(result.get("Rock").unwrap().name, "Rock");
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_rgb_statements() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5 0.1
            Kd 0.2 0.5 0.1
            Ks 0.2 0.5 0.1
            Tf 0.2 0.5 0.1
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(rock.ambient_reflectivity.r, 0.2);
        assert_eq!(rock.ambient_reflectivity.g, 0.5);
        assert_eq!(rock.ambient_reflectivity.b, 0.1);

        assert_eq!(rock.diffuse_reflectivity.r, 0.2);
        assert_eq!(rock.diffuse_reflectivity.g, 0.5);
        assert_eq!(rock.diffuse_reflectivity.b, 0.1);

        assert_eq!(rock.atmosphere_reflectivity.r, 0.2);
        assert_eq!(rock.atmosphere_reflectivity.g, 0.5);
        assert_eq!(rock.atmosphere_reflectivity.b, 0.1);

        assert_eq!(rock.transmission_filter.r, 0.2);
        assert_eq!(rock.transmission_filter.g, 0.5);
        assert_eq!(rock.transmission_filter.b, 0.1);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_rgb_statements_fallbacking_to_red() {
        let file = "
            newmtl Rock
            Ka 0.2
            Kd 0.2
            Ks 0.2
            Tf 0.2
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(rock.ambient_reflectivity.r, 0.2);
        assert_eq!(rock.ambient_reflectivity.g, 0.2);
        assert_eq!(rock.ambient_reflectivity.b, 0.2);

        assert_eq!(rock.diffuse_reflectivity.r, 0.2);
        assert_eq!(rock.diffuse_reflectivity.g, 0.2);
        assert_eq!(rock.diffuse_reflectivity.b, 0.2);

        assert_eq!(rock.atmosphere_reflectivity.r, 0.2);
        assert_eq!(rock.atmosphere_reflectivity.g, 0.2);
        assert_eq!(rock.atmosphere_reflectivity.b, 0.2);

        assert_eq!(rock.transmission_filter.r, 0.2);
        assert_eq!(rock.transmission_filter.g, 0.2);
        assert_eq!(rock.transmission_filter.b, 0.2);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_rgb_statements_fallbacking_to_red2() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(rock.ambient_reflectivity.r, 0.2);
        assert_eq!(rock.ambient_reflectivity.g, 0.5);
        assert_eq!(rock.ambient_reflectivity.b, 0.2);

        assert_eq!(rock.diffuse_reflectivity.r, 0.2);
        assert_eq!(rock.diffuse_reflectivity.g, 0.5);
        assert_eq!(rock.diffuse_reflectivity.b, 0.2);

        assert_eq!(rock.atmosphere_reflectivity.r, 0.2);
        assert_eq!(rock.atmosphere_reflectivity.g, 0.5);
        assert_eq!(rock.atmosphere_reflectivity.b, 0.2);

        assert_eq!(rock.transmission_filter.r, 0.2);
        assert_eq!(rock.transmission_filter.g, 0.5);
        assert_eq!(rock.transmission_filter.b, 0.2);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_default_illumination_model() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(rock.illumination_model, IlluminationModel::ColorOnAmbientOff);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_illumination_model() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
            illum 2
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(rock.illumination_model, IlluminationModel::HighlightOn);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_specular_highlight_exponent() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
            illum 2
            Ns 100
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(rock.specular_highlight_exponent, 100.0);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_sharpness() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
            illum 2
            Ns 100
            sharpness 100
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(rock.sharpness, 100.0);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_optical_density() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
            illum 2
            Ns 100
            Ni 0.002
";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();

        assert_eq!(rock.optical_density, 0.002);
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_not_resolve_optical_density() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
            illum 2
            Ns 100
            Ni -0.1
";

        let result = parse_mtl(file);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("'Ni' value should range between 0.001 and 10"));

    }

    #[test]
    #[rustfmt::skip]
    fn it_should_not_resolve_optical_density2() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5
            Kd 0.2 0.5
            Ks 0.2 0.5
            Tf 0.2 0.5
            illum 2
            Ns 100
            Ni 11
";

        let result = parse_mtl(file);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("'Ni' value should range between 0.001 and 10"));

    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_multiple_materials() {
        let file = "
            newmtl Rock
            Ka 0.2 0.5 0.1
            Kd 0.2 0.5 0.1
            Ks 0.2 0.5 0.1
            Tf 0.2 0.5 0.1
            illum 2
            Ns 100
            sharpness 50
            Ni 0.002

            newmtl Dirt
            Ka 0.1 0.3 0.5
            Kd 0.1 0.3 0.5
            Ks 0.1 0.3 0.5
            Tf 0.1 0.3 0.5
            illum 10
            Ns 14
            sharpness 20
            Ni 0.2

";

        let result = parse_mtl(file).expect("This should work");
        let rock = result.get("Rock").unwrap();
        let dirt = result.get("Dirt").unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(rock.ambient_reflectivity.r, 0.2);
        assert_eq!(rock.ambient_reflectivity.g, 0.5);
        assert_eq!(rock.ambient_reflectivity.b, 0.1);

        assert_eq!(rock.diffuse_reflectivity.r, 0.2);
        assert_eq!(rock.diffuse_reflectivity.g, 0.5);
        assert_eq!(rock.diffuse_reflectivity.b, 0.1);

        assert_eq!(rock.atmosphere_reflectivity.r, 0.2);
        assert_eq!(rock.atmosphere_reflectivity.g, 0.5);
        assert_eq!(rock.atmosphere_reflectivity.b, 0.1);

        assert_eq!(rock.transmission_filter.r, 0.2);
        assert_eq!(rock.transmission_filter.g, 0.5);
        assert_eq!(rock.transmission_filter.b, 0.1);

        assert_eq!(rock.specular_highlight_exponent, 100.0);
        assert_eq!(rock.sharpness, 50.0);
        assert_eq!(rock.optical_density, 0.002);
        assert_eq!(rock.illumination_model, IlluminationModel::HighlightOn);



        assert_eq!(dirt.ambient_reflectivity.r, 0.1);
        assert_eq!(dirt.ambient_reflectivity.g, 0.3);
        assert_eq!(dirt.ambient_reflectivity.b, 0.5);

        assert_eq!(dirt.diffuse_reflectivity.r, 0.1);
        assert_eq!(dirt.diffuse_reflectivity.g, 0.3);
        assert_eq!(dirt.diffuse_reflectivity.b, 0.5);

        assert_eq!(dirt.atmosphere_reflectivity.r, 0.1);
        assert_eq!(dirt.atmosphere_reflectivity.g, 0.3);
        assert_eq!(dirt.atmosphere_reflectivity.b, 0.5);

        assert_eq!(dirt.transmission_filter.r, 0.1);
        assert_eq!(dirt.transmission_filter.g, 0.3);
        assert_eq!(dirt.transmission_filter.b, 0.5);

        assert_eq!(dirt.specular_highlight_exponent, 14.0);
        assert_eq!(dirt.sharpness, 20.0);
        assert_eq!(dirt.optical_density, 0.2);
        assert_eq!(dirt.illumination_model, IlluminationModel::CastsShadows);

    }

    #[test]
    #[rustfmt::skip]
    fn it_should_resolve_42_logo() {
        let file = "
            # Blender MTL File: '42.blend'
            # Material Count: 1

            newmtl Material
            Ns 96.078431
            Ka 0.000000 0.000000 0.000000
            Kd 0.640000 0.640000 0.640000
            Ks 0.500000 0.500000 0.500000
            Ni 1.000000
            d 1.000000
            illum 2
";

        let result = parse_mtl(file).expect("This should work");
        let material = result.get("Material").unwrap();

        assert_eq!(result.len(), 1);

        assert_eq!(material.ambient_reflectivity.r, 0.0);
        assert_eq!(material.ambient_reflectivity.g, 0.0);
        assert_eq!(material.ambient_reflectivity.b, 0.0);

        assert_eq!(material.diffuse_reflectivity.r, 0.64);
        assert_eq!(material.diffuse_reflectivity.g, 0.64);
        assert_eq!(material.diffuse_reflectivity.b, 0.64);

        assert_eq!(material.atmosphere_reflectivity.r, 0.5);
        assert_eq!(material.atmosphere_reflectivity.g, 0.5);
        assert_eq!(material.atmosphere_reflectivity.b, 0.5);

        assert_eq!(material.specular_highlight_exponent, 96.078431);
        assert_eq!(material.optical_density, 1.0);
        assert_eq!(material.dissolve_factor.factor, 1.0);
        assert_eq!(material.dissolve_factor.halo, false);
        assert_eq!(material.illumination_model, IlluminationModel::HighlightOn);
    }
}
