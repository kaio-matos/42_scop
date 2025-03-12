use std::ops;

use super::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    ///
    /// Multiplies bu another quartenion.
    ///
    /// Note that quartenions are no commutative, so make sure the order of the multiplications are
    /// correct
    ///
    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}

impl ops::Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self {
        Self {
            w: self.w * rhs,
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_euler_angles(axis: Vec3, radians: f32) -> Self {
        let mut q = Self::default();
        q.w = f32::cos(radians / 2.);
        q.x = axis.x * f32::sin(radians / 2.);
        q.y = axis.y * f32::sin(radians / 2.);
        q.z = axis.z * f32::sin(radians / 2.);

        q
    }

    pub fn normalize(&self) -> Self {
        let magnitude =
            f32::sqrt(self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z);
        Self {
            w: self.w / magnitude,
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn rotate(&self, quaternion: Self) -> Self {
        quaternion * *self
    }

    pub fn rotate_mut(&mut self, quaternion: Self) -> &mut Self {
        *self = self.rotate(quaternion);
        self
    }
}
