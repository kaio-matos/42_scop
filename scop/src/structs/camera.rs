use std::{cell::RefCell, rc::Rc};

use crate::traits::*;
use basis::{
    graphics::window::Window,
    math::{self, Mat4, Vec3, VectorFunctions},
};

#[derive(Debug, Clone)]
pub struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    speed: f32,
    window: Rc<RefCell<Window>>,
}

impl Camera {
    pub fn new(
        position: Vec3,
        front: Vec3,
        up: Vec3,
        speed: f32,
        window: Rc<RefCell<Window>>,
    ) -> Self {
        Self {
            position,
            front,
            up,
            speed,
            window,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        math::Mat4::look_at(self.position, self.position.add(self.front), self.up)
    }
}

impl Controllable for Camera {
    fn get_deltatime(&self) -> f32 {
        self.window.borrow().deltatime
    }

    fn get_speed(&self) -> f32 {
        self.speed * self.get_deltatime()
    }

    fn move_forward(&mut self) {
        self.position = self.position.add(self.front.scale(self.get_speed()));
    }

    fn move_backward(&mut self) {
        self.position = self.position.subtract(self.front.scale(self.get_speed()));
    }

    fn move_left(&mut self) {
        self.position = self.position.subtract(
            self.front
                .cross(self.up)
                .normalize()
                .scale(self.get_speed()),
        );
    }

    fn move_right(&mut self) {
        self.position = self.position.add(
            self.front
                .cross(self.up)
                .normalize()
                .scale(self.get_speed()),
        );
    }

    fn move_up(&mut self) {
        self.position = self.position.add(self.up.scale(self.get_speed()));
    }

    fn move_down(&mut self) {
        self.position = self.position.subtract(self.up.scale(self.get_speed()));
    }

    fn rotate(&mut self, yaw: f32, pitch: f32) {
        let yawr = yaw.to_radians();
        let pitchr = pitch.to_radians();
        let mut direction = Vec3::default(0.);
        direction.x = yawr.cos() * pitchr.sin();
        direction.y = pitchr.sin();
        direction.z = yawr.sin() * pitchr.sin();
    }
}
