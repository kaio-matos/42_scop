use basis::math::Quaternion;

pub trait Controllable {
    fn get_speed(&self, deltatime: f32) -> f32;
    fn move_forward(&mut self, deltatime: f32);
    fn move_backward(&mut self, deltatime: f32);
    fn move_left(&mut self, deltatime: f32);
    fn move_right(&mut self, deltatime: f32);
    fn move_up(&mut self, deltatime: f32);
    fn move_down(&mut self, deltatime: f32);
    fn rotateq(&mut self, deltatime: f32, quaternion: Quaternion);
    #[allow(dead_code)]
    fn rotate(&mut self, _deltatime: f32, _yaw: f32, _pitch: f32) {}
}
