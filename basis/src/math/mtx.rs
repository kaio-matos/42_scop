use super::{Vec3, Vec4};
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Mat4 {
    pub c0: Vec4, // column 1
    pub c1: Vec4, // column 2
    pub c2: Vec4, // column 3
    pub c3: Vec4, // column 4
}

impl Mat4 {
    pub fn new(vec: Vec4) -> Self {
        Self {
            c0: Vec4::new(vec.x, 0.0, 0.0, 0.0),
            c1: Vec4::new(0.0, vec.y, 0.0, 0.0),
            c2: Vec4::new(0.0, 0.0, vec.z, 0.0),
            c3: Vec4::new(0.0, 0.0, 0.0, vec.w),
        }
    }

    pub fn identity() -> Self {
        Self {
            c0: Vec4::new(1.0, 0.0, 0.0, 0.0),
            c1: Vec4::new(0.0, 1.0, 0.0, 0.0),
            c2: Vec4::new(0.0, 0.0, 1.0, 0.0),
            c3: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn default(n: f32) -> Self {
        Self {
            c0: Vec4::new(n, n, n, n),
            c1: Vec4::new(n, n, n, n),
            c2: Vec4::new(n, n, n, n),
            c3: Vec4::new(n, n, n, n),
        }
    }

    pub fn multiply(&mut self, mat: Mat4) -> &Self {
        let mut new = Mat4::default(0.0);

        // First line
        new.c0.x = self.c0.x * mat.c0.x
            + self.c1.x * mat.c0.y
            + self.c2.x * mat.c0.z
            + self.c3.x * mat.c0.w;
        new.c1.x = self.c0.x * mat.c1.x
            + self.c1.x * mat.c1.y
            + self.c2.x * mat.c1.z
            + self.c3.x * mat.c1.w;
        new.c2.x = self.c0.x * mat.c2.x
            + self.c1.x * mat.c2.y
            + self.c2.x * mat.c2.z
            + self.c3.x * mat.c2.w;
        new.c3.x = self.c0.x * mat.c3.x
            + self.c1.x * mat.c3.y
            + self.c2.x * mat.c3.z
            + self.c3.x * mat.c3.w;

        // Second line
        new.c0.y = self.c0.y * mat.c0.x
            + self.c1.y * mat.c0.y
            + self.c2.y * mat.c0.z
            + self.c3.y * mat.c0.w;
        new.c1.y = self.c0.y * mat.c1.x
            + self.c1.y * mat.c1.y
            + self.c2.y * mat.c1.z
            + self.c3.y * mat.c1.w;
        new.c2.y = self.c0.y * mat.c2.x
            + self.c1.y * mat.c2.y
            + self.c2.y * mat.c2.z
            + self.c3.y * mat.c2.w;
        new.c3.y = self.c0.y * mat.c3.x
            + self.c1.y * mat.c3.y
            + self.c2.y * mat.c3.z
            + self.c3.y * mat.c3.w;

        // Third line
        new.c0.z = self.c0.z * mat.c0.x
            + self.c1.z * mat.c0.y
            + self.c2.z * mat.c0.z
            + self.c3.z * mat.c0.w;
        new.c1.z = self.c0.z * mat.c1.x
            + self.c1.z * mat.c1.y
            + self.c2.z * mat.c1.z
            + self.c3.z * mat.c1.w;
        new.c2.z = self.c0.z * mat.c2.x
            + self.c1.z * mat.c2.y
            + self.c2.z * mat.c2.z
            + self.c3.z * mat.c2.w;
        new.c3.z = self.c0.z * mat.c3.x
            + self.c1.z * mat.c3.y
            + self.c2.z * mat.c3.z
            + self.c3.z * mat.c3.w;

        // Fourth line
        new.c0.w = self.c0.w * mat.c0.x
            + self.c1.w * mat.c0.y
            + self.c2.w * mat.c0.z
            + self.c3.w * mat.c0.w;
        new.c1.w = self.c0.w * mat.c1.x
            + self.c1.w * mat.c1.y
            + self.c2.w * mat.c1.z
            + self.c3.w * mat.c1.w;
        new.c2.w = self.c0.w * mat.c2.x
            + self.c1.w * mat.c2.y
            + self.c2.w * mat.c2.z
            + self.c3.w * mat.c2.w;
        new.c3.w = self.c0.w * mat.c3.x
            + self.c1.w * mat.c3.y
            + self.c2.w * mat.c3.z
            + self.c3.w * mat.c3.w;

        *self = new;
        self
    }

    pub fn translate(&mut self, vec: Vec3) -> &Self {
        let mut translation_mtx = Mat4::default(0.0);
        translation_mtx.c3.x = vec.x;
        translation_mtx.c3.y = vec.y;
        translation_mtx.c3.z = vec.z;

        translation_mtx.c0.x = 1.;
        translation_mtx.c1.y = 1.;
        translation_mtx.c2.z = 1.;
        translation_mtx.c3.w = 1.;

        *self = *translation_mtx.multiply(*self);
        self
    }

    // Needs a refactor to use quartenions
    pub fn rotate(&mut self, radians: f32, r: Vec3) -> &Self {
        let mut rotation_mtx = Mat4::default(0.0);

        let cos = f32::cos(radians);
        let sin = f32::sin(radians);

        ////
        //// prevents gimbal lock most of the times
        ////
        // rotation_mtx.c0.x = cos + (r.x * r.x) * (1. - cos);
        // rotation_mtx.c0.y = r.y * r.x * (1. - cos) + r.z * sin;
        // rotation_mtx.c0.z = r.z * r.x * (1. - cos) - r.y * sin;
        // rotation_mtx.c0.w = 0.;
        //
        // rotation_mtx.c1.x = r.x * r.y * (1. - cos) - r.z * sin;
        // rotation_mtx.c1.y = cos + (r.y * r.y) * (1. - cos);
        // rotation_mtx.c1.z = r.z * r.y * (1. - cos) + r.x * sin;
        // rotation_mtx.c1.w = 0.;
        //
        // rotation_mtx.c2.x = r.x * r.z * (1. - cos) + r.y * sin;
        // rotation_mtx.c2.y = r.y * r.z * (1. - cos) - r.x * sin;
        // rotation_mtx.c2.z = cos + (r.z * r.z) * (1. - cos);
        // rotation_mtx.c2.w = 0.;
        //
        // rotation_mtx.c3.x = 0.;
        // rotation_mtx.c3.y = 0.;
        // rotation_mtx.c3.z = 0.;
        // rotation_mtx.c3.w = 1.;

        if r.x > 0.0 {
            rotation_mtx.c0.x = 1.;

            rotation_mtx.c1.y = cos;
            rotation_mtx.c1.z = sin;

            rotation_mtx.c2.y = -sin;
            rotation_mtx.c2.z = cos;

            rotation_mtx.c3.w = 1.;
        } else if r.y > 0.0 {
            rotation_mtx.c0.x = cos;
            rotation_mtx.c0.z = -sin;

            rotation_mtx.c1.y = 1.;

            rotation_mtx.c2.x = sin;
            rotation_mtx.c2.z = cos;

            rotation_mtx.c3.w = 1.;
        } else if r.z > 0.0 {
            rotation_mtx.c0.x = cos;
            rotation_mtx.c0.y = sin;

            rotation_mtx.c1.x = -sin;
            rotation_mtx.c1.y = cos;

            rotation_mtx.c2.z = 1.;

            rotation_mtx.c3.w = 1.;
        }

        *self = *rotation_mtx.multiply(*self);
        self
    }

    pub fn scale(&mut self, vec: Vec3) -> &Self {
        let mut scale_mtx = Mat4::default(0.0);
        scale_mtx.c0.x = vec.x;
        scale_mtx.c1.y = vec.y;
        scale_mtx.c2.z = vec.z;
        scale_mtx.c3.w = 1.0;

        *self = *scale_mtx.multiply(*self);
        self
    }

    pub fn as_f32_ptr(&self) -> *const f32 {
        // self as *const Mat4 as *const f32

        ptr::from_ref(&self.c0.x) as *const f32
    }
}

impl std::fmt::Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat4 {{\n")?;
        write!(f, "  c0: {:?}\n", self.c0)?;
        write!(f, "  c1: {:?}\n", self.c1)?;
        write!(f, "  c2: {:?}\n", self.c2)?;
        write!(f, "  c3: {:?}\n", self.c3)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_instantiate_mat4() {
        let result = Mat4::default(1.0);

        // line 1
        assert_eq!(result.c0.x, 1.0);
        assert_eq!(result.c0.y, 1.0);
        assert_eq!(result.c0.z, 1.0);
        assert_eq!(result.c0.w, 1.0);

        // line 2
        assert_eq!(result.c1.x, 1.0);
        assert_eq!(result.c1.y, 1.0);
        assert_eq!(result.c1.z, 1.0);
        assert_eq!(result.c1.w, 1.0);

        // line 3
        assert_eq!(result.c2.x, 1.0);
        assert_eq!(result.c2.y, 1.0);
        assert_eq!(result.c2.z, 1.0);
        assert_eq!(result.c2.w, 1.0);

        // line 4
        assert_eq!(result.c3.x, 1.0);
        assert_eq!(result.c3.y, 1.0);
        assert_eq!(result.c3.z, 1.0);
        assert_eq!(result.c3.w, 1.0);
    }

    #[test]
    fn should_be_able_to_instantiate_mat4_with_new() {
        let result = Mat4::new(Vec4::default(1.0));

        // line 1
        assert_eq!(result.c0.x, 1.0);
        assert_eq!(result.c0.y, 0.0);
        assert_eq!(result.c0.z, 0.0);
        assert_eq!(result.c0.w, 0.0);

        // line 2
        assert_eq!(result.c1.x, 0.0);
        assert_eq!(result.c1.y, 1.0);
        assert_eq!(result.c1.z, 0.0);
        assert_eq!(result.c1.w, 0.0);

        // line 3
        assert_eq!(result.c2.x, 0.0);
        assert_eq!(result.c2.y, 0.0);
        assert_eq!(result.c2.z, 1.0);
        assert_eq!(result.c2.w, 0.0);

        // line 4
        assert_eq!(result.c3.x, 0.0);
        assert_eq!(result.c3.y, 0.0);
        assert_eq!(result.c3.z, 0.0);
        assert_eq!(result.c3.w, 1.0);
    }

    #[test]
    fn should_be_able_to_multiply_mtx4() {
        let mut first = Mat4::default(0.0);
        first.c0.x = 1.0;
        first.c0.y = 0.0;
        first.c0.z = 2.0;
        first.c0.w = 1.0;

        first.c1.x = 5.0;
        first.c1.y = 1.0;
        first.c1.z = 5.0;
        first.c1.w = 2.0;

        first.c2.x = 1.0;
        first.c2.y = 3.0;
        first.c2.z = 4.0;
        first.c2.w = 1.0;

        first.c3.x = 7.0;
        first.c3.y = 6.0;
        first.c3.z = 3.0;
        first.c3.w = 15.0;

        let mut second = Mat4::default(0.0);
        second.c0.x = 2.0;
        second.c0.y = 6.0;
        second.c0.z = 4.0;
        second.c0.w = 2.0;

        second.c1.x = 4.0;
        second.c1.y = 1.0;
        second.c1.z = 3.0;
        second.c1.w = 6.0;

        second.c2.x = 3.0;
        second.c2.y = 14.0;
        second.c2.z = 4.0;
        second.c2.w = 1.0;

        second.c3.x = 1.0;
        second.c3.y = 3.0;
        second.c3.z = 6.0;
        second.c3.w = 5.0;

        first.multiply(second);

        // column 1
        assert_eq!(first.c0.x, 50.0);
        assert_eq!(first.c0.y, 30.0);
        assert_eq!(first.c0.z, 56.0);
        assert_eq!(first.c0.w, 48.0);

        // column 2
        assert_eq!(first.c1.x, 54.0);
        assert_eq!(first.c1.y, 46.0);
        assert_eq!(first.c1.z, 43.0);
        assert_eq!(first.c1.w, 99.0);

        // column 3
        assert_eq!(first.c2.x, 84.0);
        assert_eq!(first.c2.y, 32.0);
        assert_eq!(first.c2.z, 95.0);
        assert_eq!(first.c2.w, 50.0);

        // column 4
        assert_eq!(first.c3.x, 57.0);
        assert_eq!(first.c3.y, 51.0);
        assert_eq!(first.c3.z, 56.0);
        assert_eq!(first.c3.w, 88.0);
    }

    #[test]
    fn should_be_able_to_translate_mtx4() {
        let translate_vector = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let mut first = Mat4::default(0.0);
        first.c0.x = 1.0;
        first.c0.y = 0.0;
        first.c0.z = 1.0;
        first.c0.w = 1.0;

        first.c1.x = 1.0;
        first.c1.y = 1.0;
        first.c1.z = 1.0;
        first.c1.w = 1.0;

        first.c2.x = 1.0;
        first.c2.y = 1.0;
        first.c2.z = 1.0;
        first.c2.w = 1.0;

        first.c3.x = 1.0;
        first.c3.y = 2.0;
        first.c3.z = 3.0;
        first.c3.w = 1.0;

        first.translate(translate_vector);

        // line 1
        assert_eq!(first.c0.x, 2.0);
        assert_eq!(first.c0.y, 2.0);
        assert_eq!(first.c0.z, 4.0);
        assert_eq!(first.c0.w, 1.0);

        // line 2
        assert_eq!(first.c1.x, 2.0);
        assert_eq!(first.c1.y, 3.0);
        assert_eq!(first.c1.z, 4.0);
        assert_eq!(first.c1.w, 1.0);

        // line 3
        assert_eq!(first.c2.x, 2.0);
        assert_eq!(first.c2.y, 3.0);
        assert_eq!(first.c2.z, 4.0);
        assert_eq!(first.c2.w, 1.0);

        // line 4
        assert_eq!(first.c3.x, 2.0);
        assert_eq!(first.c3.y, 4.0);
        assert_eq!(first.c3.z, 6.0);
        assert_eq!(first.c3.w, 1.0);
    }

    #[test]
    fn should_be_able_to_rotate_around_z_axis_mtx4() {
        let rotation_vector = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let radians = 90.0_f32.to_radians();

        let mut first = Mat4::default(0.0);
        first.c0 = Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        first.c1 = Vec4 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        first.c2 = Vec4 {
            x: 2.5,
            y: 5.0,
            z: 0.0,
            w: 0.0,
        };
        first.c3 = Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        first.rotate(radians, rotation_vector);

        // line 1
        assert_eq!(first.c0.x, 0.0);
        assert_eq!(first.c0.y, 0.0);
        assert_eq!(first.c0.z, 0.0);
        assert_eq!(first.c0.w, 0.0);

        // line 2
        assert_eq!(first.c1.x, -2.1855695e-7);
        assert_eq!(first.c1.y, 5.0);
        assert_eq!(first.c1.z, 0.0);
        assert_eq!(first.c1.w, 0.0);

        // line 3
        assert_eq!(first.c2.x, -5.0);
        assert_eq!(first.c2.y, 2.4999998);
        assert_eq!(first.c2.z, 0.0);
        assert_eq!(first.c2.w, 0.0);

        // line 4
        assert_eq!(first.c3.x, 0.0);
        assert_eq!(first.c3.y, 0.0);
        assert_eq!(first.c3.z, 0.0);
        assert_eq!(first.c3.w, 0.0);
    }

    #[test]
    fn should_be_able_to_scale_mtx4() {
        let scale_vector = Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let mut first = Mat4::default(0.0);
        first.c0.x = 1.0;
        first.c0.y = 2.0;
        first.c0.z = 0.0;
        first.c0.w = 2.0;

        first.c1.x = 2.0;
        first.c1.y = 4.0;
        first.c1.z = 1.0;
        first.c1.w = 7.0;

        first.c2.x = 3.0;
        first.c2.y = 2.0;
        first.c2.z = 5.0;
        first.c2.w = 2.0;

        first.c3.x = 8.0;
        first.c3.y = 6.0;
        first.c3.z = 2.0;
        first.c3.w = 5.0;

        first.scale(scale_vector);

        // line 1
        assert_eq!(first.c0.x, 2.0);
        assert_eq!(first.c0.y, 4.0);
        assert_eq!(first.c0.z, 0.0);
        assert_eq!(first.c0.w, 2.0);

        // line 2
        assert_eq!(first.c1.x, 4.0);
        assert_eq!(first.c1.y, 8.0);
        assert_eq!(first.c1.z, 2.0);
        assert_eq!(first.c1.w, 7.0);

        // line 3
        assert_eq!(first.c2.x, 6.0);
        assert_eq!(first.c2.y, 4.0);
        assert_eq!(first.c2.z, 10.0);
        assert_eq!(first.c2.w, 2.0);

        // line 4
        assert_eq!(first.c3.x, 16.0);
        assert_eq!(first.c3.y, 12.0);
        assert_eq!(first.c3.z, 4.0);
        assert_eq!(first.c3.w, 5.0);
    }
}