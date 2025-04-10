use basis::graphics;
use basis::math::Quaternion;

use crate::math;
use crate::traits::{Controllable, EntityLifetime};

use super::Object;

pub struct Cube {
    pub object: Object,
}

impl Controllable for Cube {
    fn get_speed(&self, deltatime: f32) -> f32 {
        30. * deltatime
    }

    fn move_forward(&mut self, deltatime: f32) {
        self.object.position.z -= self.get_speed(deltatime);
    }

    fn move_backward(&mut self, deltatime: f32) {
        self.object.position.z += self.get_speed(deltatime);
    }

    fn move_left(&mut self, deltatime: f32) {
        self.object.position.x -= self.get_speed(deltatime);
    }

    fn move_right(&mut self, deltatime: f32) {
        self.object.position.x += self.get_speed(deltatime);
    }

    fn move_up(&mut self, deltatime: f32) {
        self.object.position.y += self.get_speed(deltatime);
    }

    fn move_down(&mut self, deltatime: f32) {
        self.object.position.y -= self.get_speed(deltatime);
    }

    fn rotate(&mut self, _deltatime: f32, _yaw: f32, _pitch: f32) {}

    fn rotateq(&mut self, deltatime: f32, quaternion: Quaternion) {
        self.object
            .rotation
            .rotate_mut(quaternion * self.get_speed(deltatime));
    }
}

impl EntityLifetime for Cube {
    fn get_object(&mut self) -> Option<&mut Object> {
        Some(&mut self.object)
    }

    fn update(&mut self, window: &mut basis::graphics::window::Window) {
        if window.on_key_hold(graphics::glfw::Key::Up, graphics::glfw::Modifiers::empty()) {
            self.move_up(window.deltatime)
        }
        if window.on_key_hold(
            graphics::glfw::Key::Down,
            graphics::glfw::Modifiers::empty(),
        ) {
            self.move_down(window.deltatime)
        }
        if window.on_key_hold(
            graphics::glfw::Key::Left,
            graphics::glfw::Modifiers::empty(),
        ) {
            self.move_left(window.deltatime)
        }
        if window.on_key_hold(
            graphics::glfw::Key::Right,
            graphics::glfw::Modifiers::empty(),
        ) {
            self.move_right(window.deltatime)
        }
        if window.on_key_hold(
            graphics::glfw::Key::PageUp,
            graphics::glfw::Modifiers::empty(),
        ) {
            self.move_forward(window.deltatime)
        }
        if window.on_key_hold(
            graphics::glfw::Key::PageDown,
            graphics::glfw::Modifiers::empty(),
        ) {
            self.move_backward(window.deltatime)
        }

        if window.on_key_hold(graphics::glfw::Key::Up, graphics::glfw::Modifiers::Control) {
            self.rotateq(
                window.deltatime,
                Quaternion::from_euler_angles(math::Vec3::new(0.1, 0.0, 0.0), 5_f32.to_radians()),
            );
        }

        if window.on_key_hold(
            graphics::glfw::Key::Down,
            graphics::glfw::Modifiers::Control,
        ) {
            self.rotateq(
                window.deltatime,
                Quaternion::from_euler_angles(math::Vec3::new(-0.1, 0.0, 0.0), 5_f32.to_radians()),
            );
        }
        if window.on_key_hold(
            graphics::glfw::Key::Left,
            graphics::glfw::Modifiers::Control,
        ) {
            self.rotateq(
                window.deltatime,
                Quaternion::from_euler_angles(math::Vec3::new(0.0, -0.1, 0.0), 5_f32.to_radians()),
            );
        }
        if window.on_key_hold(
            graphics::glfw::Key::Right,
            graphics::glfw::Modifiers::Control,
        ) {
            self.rotateq(
                window.deltatime,
                Quaternion::from_euler_angles(math::Vec3::new(0.0, 0.1, 0.0), 5_f32.to_radians()),
            );
        }
    }
}
