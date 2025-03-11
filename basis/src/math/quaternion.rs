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

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
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

    ///
    /// Multiplies bu another quartenion.
    ///
    /// Note that quartenions are no commutative, so make sure the order of the multiplications are
    /// correct
    ///
    pub fn mul(&self, q: Self) -> Self {
        Self {
            w: self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            x: self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
            y: self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
            z: self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
        }
    }

    pub fn rotate(&self, axis: Vec3, radians: f32) -> Self {
        let mut temporary = Self::default();
        temporary.w = f32::cos(radians / 2.);
        temporary.x = axis.x * f32::sin(radians / 2.);
        temporary.y = axis.y * f32::sin(radians / 2.);
        temporary.z = axis.z * f32::sin(radians / 2.);

        temporary.mul(*self)
    }

    pub fn rotate_mut(&mut self, axis: Vec3, radians: f32) -> &mut Self {
        *self = self.rotate(axis, radians);
        self
    }
}
