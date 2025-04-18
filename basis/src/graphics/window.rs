use std::collections::HashMap;

use glfw::{Action, Context, GlfwReceiver, Key, Modifiers, WindowEvent};

#[derive(Debug, Hash, Eq, PartialEq)]
struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
}

#[derive(Debug)]
pub struct Window {
    pub glfw: glfw::Glfw,
    pub deltatime: f32,
    pub events: Vec<WindowEvent>,

    hold_keys: HashMap<KeyEvent, bool>,
    window_handle: glfw::PWindow,
    raw_events: GlfwReceiver<(f64, WindowEvent)>,
    last_frame: f32,
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
            deltatime: 0.0,
            events: Vec::default(),

            hold_keys: HashMap::default(),
            window_handle: window,
            raw_events: events,
            last_frame: 0.0,
        }
    }

    pub fn compute_deltatime(&mut self) {
        let current_frame = self.glfw.get_time() as f32;
        self.deltatime = current_frame - self.last_frame;
        self.last_frame = current_frame;
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.window_handle.get_size()
    }

    pub fn update<F>(&mut self, on_event: &mut F)
    where
        F: FnMut(&WindowEvent),
    {
        self.process_events(on_event);
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    pub fn on_key_release(&mut self, key: glfw::Key, modifiers: glfw::Modifiers) -> bool {
        let found = self.events.iter().find(|event| {
            match event {
                WindowEvent::Key(k, _, action, m) => {
                    if k == &key && m == &modifiers && action == &Action::Release {
                        return true;
                    }
                }
                _ => {}
            }
            false
        });
        found.is_some()
    }

    pub fn on_key_press(&mut self, key: glfw::Key, modifiers: glfw::Modifiers) -> bool {
        let found = self.events.iter().find(|event| {
            match event {
                WindowEvent::Key(k, _, action, m) => {
                    if k == &key && m == &modifiers && action == &Action::Press {
                        return true;
                    }
                }
                _ => {}
            }
            false
        });
        found.is_some()
    }

    pub fn on_key_hold(&self, key: glfw::Key, modifiers: glfw::Modifiers) -> bool {
        let is_pressing = self.hold_keys.get(&KeyEvent { key, modifiers });

        match is_pressing {
            Some(value) => *value,
            None => false,
        }
    }

    fn process_events<F>(&mut self, on_event: &mut F)
    where
        F: FnMut(&WindowEvent),
    {
        self.events.clear();
        for (_, event) in glfw::flush_messages(&self.raw_events) {
            self.events.push(event.clone());
            match event {
                glfw::WindowEvent::Key(k, _, action, m) => {
                    let key = KeyEvent {
                        key: k,
                        modifiers: m,
                    };

                    if action == Action::Press {
                        self.hold_keys.insert(key, true);
                    } else if action == Action::Release {
                        // make sure we release all of them, even if the key was pressed by using a
                        // modifier
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::empty(),
                            },
                            false,
                        );
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::Alt,
                            },
                            false,
                        );
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::Shift,
                            },
                            false,
                        );
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::Control,
                            },
                            false,
                        );
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::Super,
                            },
                            false,
                        );
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::NumLock,
                            },
                            false,
                        );
                        self.hold_keys.insert(
                            KeyEvent {
                                key: k,
                                modifiers: Modifiers::CapsLock,
                            },
                            false,
                        );
                        self.hold_keys.insert(key, false);
                    }
                }
                _ => {}
            }
            on_event(&event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
