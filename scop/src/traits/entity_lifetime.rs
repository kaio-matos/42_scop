use crate::structs::Object;

pub trait EntityLifetime {
    fn get_object(&mut self) -> Option<&mut Object> {
        None
    }

    fn setup(&mut self) {}

    fn update(&mut self, _window: &mut basis::graphics::window::Window) {}
}
