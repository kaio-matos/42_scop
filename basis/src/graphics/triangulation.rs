pub mod _2d {
    pub mod fan {
        use crate::graphics::wavefront::obj::VertexDataReference;

        ///
        /// Triangulate a polygon using the fan algorithm, which is a simple and fast algorithm.
        /// However, it only works for convex polygons.
        ///
        pub fn triangulate(
            vertex_references: &mut Vec<VertexDataReference>,
        ) -> Vec<VertexDataReference> {
            let mut result = Vec::new();

            // if it is not a triangle, but a quad or n-gon
            if vertex_references.len() > 3 {
                // then convert to triangles
                for i in 1..vertex_references.len() - 1 {
                    result.push(vertex_references[0].clone());
                    result.push(vertex_references[i].clone());
                    result.push(vertex_references[i + 1].clone());
                }
            } else {
                result = vertex_references.to_vec();
            }

            result
        }
    }

    pub mod ear_clipping {
        use crate::graphics::wavefront::obj::VertexDataReference;

        ///
        /// Triangulate a polygon using the ear clipping algorithm.
        /// It works for both convex and concave polygons.
        ///
        pub fn triangulate(
            _vertex_references: &mut Vec<VertexDataReference>,
        ) -> Vec<VertexDataReference> {
            todo!();
        }
    }
}

pub mod _3d {}
