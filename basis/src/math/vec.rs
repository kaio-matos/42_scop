#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn default(n: f32) -> Self {
        Self {
            x: n,
            y: n,
            z: n,
            w: n,
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn default(n: f32) -> Self {
        Self { x: n, y: n, z: n }
    }

    pub fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn normalize(&self) -> Self {
        let length_of_v = f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z));

        Self {
            x: self.x / length_of_v,
            y: self.y / length_of_v,
            z: self.z / length_of_v,
        }
    }

    pub fn subtract(&self, v: Vec3) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn cross(&self, v: Vec3) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
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
