use glfw::{Action, Context, GlfwReceiver, Key, WindowEvent};

pub struct Window {
    pub glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update<F>(&mut self, on_event: &mut F)
    where
        F: FnMut(&WindowEvent),
    {
        self.process_events(on_event);
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events<F>(&mut self, on_event: &mut F)
    where
        F: FnMut(&WindowEvent),
    {
        for (_, event) in glfw::flush_messages(&self.events) {
            on_event(&event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
