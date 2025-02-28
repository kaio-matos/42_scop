pub trait Controllable {
    fn get_deltatime(&self) -> f32;
    fn get_speed(&self) -> f32;
    fn move_forward(&mut self);
    fn move_backward(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn rotate(&mut self, yaw: f32, pitch: f32);
}
