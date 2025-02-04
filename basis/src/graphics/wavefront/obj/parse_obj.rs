use super::helpers;
use super::structs::{ParseError, OBJ};

pub fn parse_obj(data: &str) -> Result<OBJ, ParseError> {
    let mut obj = OBJ::default();

    let lines = data.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty());
    let mut previous_line = Option::None;
    let mut current_line: usize = 1;
    for line in lines {
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
            //
            // Vertex data
            //
            "v" => {
                // Parse vertex
                let result = helpers::parse_vertice(&mut tokens, current_line)?;
                obj.vertices.push(result);
                Ok(())
            }
            "vt" => {
                // Parse vertex texture
                let result = helpers::parse_vertice_texture(&mut tokens, current_line)?;
                obj.vertices_texture.push(result);
                Ok(())
            }
            "vn" => {
                // Parse vertex normal
                let result = helpers::parse_vertice_normal(&mut tokens, current_line)?;
                obj.vertices_normal.push(result);
                Ok(())
            }
            "vp" => {
                // Parse parameter space vertices
                let result = helpers::parse_vertice_parameter_space(&mut tokens, current_line)?;
                obj.vertices_parameter_space.push(result);
                Ok(())
            }
            //
            // Free-form curve/surface attributes
            //
            "cstype" => {
                // Parse curve or surface type
                todo!("Implement cstype");
            }
            "deg" => {
                // Parse degree
                todo!("Implement deg");
            }
            "bmat" => {
                // Parse basis matrix
                todo!("Implement bmat");
            }
            "step" => {
                // Parse step size
                todo!("Implement step");
            }
            //
            // Elements
            //
            "p" => {
                // Parse point
                todo!("Implement p");
            }
            "l" => {
                // Parse line
                todo!("Implement l");
            }
            "f" => {
                // Parse face
                let result = helpers::parse_face(&mut tokens, previous_line, current_line)?;
                obj.faces.push(result);
                Ok(())
            }
            "curv" => {
                // Parse curve
                todo!("Implement curv");
            }
            "curv2" => {
                // Parse 2D curve
                todo!("Implement curv2");
            }
            "surf" => {
                // Parse surface
                todo!("Implement surf");
            }
            //
            // Free-form curve/surface body statements
            //
            "parm" => {
                // Parse parameter values
                todo!("Implement parm");
            }
            "trim" => {
                // Parse outer trimming loop
                todo!("Implement trim");
            }
            "hole" => {
                // Parse inner trimming loop
                todo!("Implement hole");
            }
            "scrv" => {
                // Parse special curve
                todo!("Implement srcv");
            }
            "sp" => {
                // Parse special point
                todo!("Implement sp");
            }
            "end" => {
                // Parse end statement
                todo!("Implement end");
            }
            //
            // Connectivity between free-form surfaces
            //
            "con" => {
                // Parse connect
                todo!("Implement con");
            }
            //
            // Grouping
            //
            "g" => {
                // Parse group name
                todo!("Implement g");
            }
            "s" => {
                // Parse smoothing group
                todo!("Implement s");
            }
            "mg" => {
                // Parse merging group
                todo!("Implement mg");
            }
            "o" => {
                // Parse object name
                tokens.next().map(|name| obj.name = Some(name.to_string()));
                Ok(())
            }
            //
            // Display/render attributes
            //
            "bevel" => {
                // Parse bevel interpolation
                todo!("Implement bevel");
            }
            "c_interp" => {
                // Parse color interpolation
                todo!("Implement c_interp");
            }
            "d_interp" => {
                // Parse dissolve interpolation
                todo!("Implement d_interp");
            }
            "lod" => {
                // Parse level of detail
                todo!("Implement lod");
            }
            "usemtl" => {
                // Skip... This is done during the face parsing
                if tokens.len() > 2 {
                    return Err(ParseError::InvalidFaceMaterial(
                        current_line,
                        "You can only specify one material".to_string(),
                    ));
                }
                Ok(())
            }
            "mtllib" => {
                // Parse material library
                let mtllib = tokens
                    .clone()
                    .filter(|s| s != &"mtllib")
                    .map(|s| s.to_string())
                    .collect();
                obj.materials_names = mtllib;
                Ok(())
            }
            "shadow_obj" => {
                // Parse shadow casting
                todo!("Implement shadow_obj");
            }
            "trace_obj" => {
                // Parse ray tracing
                todo!("Implement trace_obj");
            }
            "ctech" => {
                // Parse curve approximation technique
                todo!("Implement c_tech");
            }
            "stech" => {
                // Parse surface approximation technique
                todo!("Implement stech");
            }
            "#" => {
                // Ignore comments
                Ok(())
            }
            unknown => Err(ParseError::InvalidToken(
                current_line,
                format!("Unknown token: '{unknown}'"),
            )),
        }?;
        current_line += 1;
        previous_line = Some(line);
    }
    Ok(obj)
}

#[cfg(test)]
mod tests {
    use crate::graphics::wavefront::obj::structs::{
        FaceSide, Vertice, VerticeNormal, VerticeParameterSpace, VerticeTexture,
    };

    use super::*;

    #[test]
    fn it_should_be_able_to_ignore_comments() {
        let file = "
            # This is a comment

            mtllib cube.mtl
            o cube39
            # v 0.232406 -1.216630 1.133818
            # v 0.232406 -0.745504 2.843098
            # testing
            # v -0.227475 -0.745504 2.843098
            # v -0.227475 -1.216630 1.133818
            # v 0.232407 1.119982 1.133819
            # v 0.232406 1.119982 1.602814
";

        let result = parse_obj(file).expect("This should work");

        assert_eq!(result.vertices.len(), 0);
        assert_eq!(result.vertices_texture.len(), 0);
        assert_eq!(result.vertices_normal.len(), 0);
        assert_eq!(result.vertices_parameter_space.len(), 0);
    }

    #[test]
    fn it_should_be_able_parse_name() {
        let file = "
            # This is a comment

            mtllib cube.mtl
            o cube39
            v 0.232406 -1.216630 1.133818
            v 0.232406 -0.745504 2.843098
";

        let result = parse_obj(file).expect("This should work");

        assert_eq!(result.name, Some("cube39".to_string()));
    }

    #[test]
    fn it_should_be_able_parse_mtllib() {
        let file = "
            # This is a comment

            mtllib cube.mtl testing.mtl
            o cube39
            v 0.232406 -1.216630 1.133818
            v 0.232406 -0.745504 2.843098
";

        let result = parse_obj(file).expect("This should work");

        assert_eq!(
            result.materials_names,
            vec!["cube.mtl".to_string(), "testing.mtl".to_string()]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_store_vertices() {
        let file = "
            # This is a comment

            mtllib cube.mtl
            o cube39
            v      -5.000000       5.000000       0.000000
            v      -5.000000      -5.000000       0.000000
            v       5.000000      -5.000000       0.000000
            v       5.000000       5.000000       0.000000
            vt     -5.000000       5.000000       0.000000
            vt     -5.000000      -5.000000       0.000000
            vt      5.000000      -5.000000       0.000000
            vt      5.000000       5.000000       0.000000
            vn      0.000000       0.000000       1.000000
            vn      0.000000       0.000000       1.000000
            vn      0.000000       0.000000       1.000000
            vn      0.000000       0.000000       1.000000
            vp      0.210000       3.590000
            vp      0.000000       0.000000
            vp      1.000000       0.000000
            vp      0.500000       0.500000
";

        let result = parse_obj(file).expect("This should work");

        assert_eq!(result.vertices.len(), 4);
        assert_eq!(result.vertices_texture.len(), 4);
        assert_eq!(result.vertices_normal.len(), 4);
        assert_eq!(result.vertices_parameter_space.len(), 4);

        assert_eq!(result.vertices[0], Vertice::new(-5.0, 5.0, 0.0, 1.0));
        assert_eq!(result.vertices[1], Vertice::new(-5.0, -5.0, 0.0, 1.0));
        assert_eq!(result.vertices[2], Vertice::new(5.0, -5.0, 0.0, 1.0));
        assert_eq!(result.vertices[3], Vertice::new(5.0, 5.0, 0.0, 1.0));

        assert_eq!(result.vertices_texture[0], VerticeTexture::new(-5.0, 5.0, 0.0));
        assert_eq!(result.vertices_texture[1], VerticeTexture::new(-5.0, -5.0, 0.0));
        assert_eq!(result.vertices_texture[2], VerticeTexture::new(5.0, -5.0, 0.0));
        assert_eq!(result.vertices_texture[3], VerticeTexture::new(5.0, 5.0, 0.0));

        assert_eq!(result.vertices_normal[0], VerticeNormal::new(0.0, 0.0, 1.0));
        assert_eq!(result.vertices_normal[1], VerticeNormal::new(0.0, 0.0, 1.0));
        assert_eq!(result.vertices_normal[2], VerticeNormal::new(0.0, 0.0, 1.0));
        assert_eq!(result.vertices_normal[3], VerticeNormal::new(0.0, 0.0, 1.0));

        assert_eq!(result.vertices_parameter_space[0], VerticeParameterSpace::new(0.21, 3.59, 1.0));
        assert_eq!(result.vertices_parameter_space[1], VerticeParameterSpace::new(0.0, 0.0, 1.0));
        assert_eq!(result.vertices_parameter_space[2], VerticeParameterSpace::new(1.0, 0.0, 1.0));
        assert_eq!(result.vertices_parameter_space[3], VerticeParameterSpace::new(0.5, 0.5, 1.0));
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_store_faces() {
        let file = "
            # This is a comment

            mtllib cube.mtl
            o cube39
            v      -5.000000       5.000000       0.000000
            vt     -5.000000       5.000000       0.000000
            vn      0.000000       0.000000       1.000000
            vp      0.210000       3.590000
            f 1/1/1 2/2/2 3/3/3 4/4/4
            f 1//1 2//2 3//3 4//4
";

        let result = parse_obj(file).expect("This should work");

        assert_eq!(result.faces[0].sides[0], FaceSide::new(1, 1, 1));
        assert_eq!(result.faces[0].sides[1], FaceSide::new(2, 2, 2));
        assert_eq!(result.faces[0].sides[2], FaceSide::new(3, 3, 3));
        assert_eq!(result.faces[0].sides[3], FaceSide::new(4, 4, 4));

        assert_eq!(result.faces[1].sides[0], FaceSide::new(1, 0, 1));
        assert_eq!(result.faces[1].sides[1], FaceSide::new(2, 0, 2));
        assert_eq!(result.faces[1].sides[2], FaceSide::new(3, 0, 3));
        assert_eq!(result.faces[1].sides[3], FaceSide::new(4, 0, 4));
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_fail_to_store_faces() {
        let file = "
            # This is a comment

            mtllib cube.mtl
            o cube39
            v      -5.000000       5.000000       0.000000
            vt     -5.000000       5.000000       0.000000
            vn      0.000000       0.000000       1.000000
            vp      0.210000       3.590000
            f 1/1/1 2/2/2 3/3/3 4/4/4
            f 1//1 2//2 3//3 4//4
            f 1/1/1 2/2/2 3//3 4//4
";

        let result = parse_obj(file);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Illegal to give vertex texture for some vertices, but not all"));
    }

    #[test]
    #[rustfmt::skip]
    fn it_should_store_face_with_materials() {
        let file = "
            # This is a comment

            mtllib cube.mtl
            o cube39
            v      -5.000000       5.000000       0.000000
            vt     -5.000000       5.000000       0.000000
            vn      0.000000       0.000000       1.000000
            vp      0.210000       3.590000
            usemtl 4bed15
            f 1/4/1 2/3/2 3/2/3 4/1/4 
            usemtl 2daec2
            f 2/8/2 5/7/5 6/6/6 3/5/3 
            f 5/12/5 7/11/7 8/10/8 6/9/6 
            f 4/24/4 3/23/3 6/22/6 8/21/8 
            usemtl 4602e3
            f 7/16/7 1/15/1 4/14/4 8/13/8 
            usemtl c41dde
            f 7/20/7 5/19/5 2/18/2 1/17/1 
";

        let result = parse_obj(file).expect("This should work");

        assert_eq!(result.faces.len(), 6);
        assert_eq!(result.faces[0].material, Some("4bed15".to_string()));
        assert_eq!(result.faces[1].material, Some("2daec2".to_string()));
        assert_eq!(result.faces[2].material, None);
        assert_eq!(result.faces[3].material, None);
        assert_eq!(result.faces[4].material, Some("4602e3".to_string()));
        assert_eq!(result.faces[5].material, Some("c41dde".to_string()));
    }
}
