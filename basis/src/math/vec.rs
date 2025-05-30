use std::ops;

pub trait VectorFunctions<T>: ops::Add<T> + ops::Sub<T> + ops::Mul<T> {
    fn negate(&self) -> Self;

    ///
    /// Transforms a vector into a unit vector (vector with length 1)
    ///
    fn normalize(&self) -> Self;
    fn scale(&self, n: f32) -> Self;

    ///
    /// Cross product of two vectors
    ///
    /// The cross product of two vectors is a vector that is perpendicular to the plane formed by
    /// the two vectors.
    ///
    /// If the vectors are parallel, the cross product is the zero vector.
    ///
    fn cross(&self, v: Self) -> Self;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn splat(n: f32) -> Self {
        Self {
            x: n,
            y: n,
            z: n,
            w: n,
        }
    }
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl VectorFunctions<Vec4> for Vec4 {
    fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    fn normalize(&self) -> Self {
        let length = f32::sqrt(
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w),
        );

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }

    fn scale(&self, n: f32) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
            w: self.w * n,
        }
    }

    fn cross(&self, v: Vec4) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
            w: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn splat(n: f32) -> Self {
        Self { x: n, y: n, z: n }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl VectorFunctions<Vec3> for Vec3 {
    fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn normalize(&self) -> Self {
        if self.x == 0.0 && self.y == 0.0 && self.z == 0.0 {
            return *self;
        }
        let length = f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z));

        if length == 0.0 {
            return *self;
        }

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    fn scale(&self, n: f32) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }

    fn cross(&self, v: Vec3) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_able_to_instantiate_vec4() {
        let result = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
        assert_eq!(result.w, 4.0);
    }
}
