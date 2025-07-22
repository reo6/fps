use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use winit::event::{DeviceEvent, ElementState, Event, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

pub mod input_system;
pub use input_system::InputSystem;

pub mod camera;
pub use camera::FPSCameraController;

pub mod camera_system;
pub use camera_system::CameraSystem;

pub struct Input<A: Copy + Eq + Hash> {
    pressed_keys: HashSet<KeyCode>,
    pressed_once: HashSet<KeyCode>,

    keymap: HashMap<KeyCode, A>,
    pressed_actions: HashSet<A>,
    pressed_actions_once: HashSet<A>,

    mouse_delta: (f64, f64),
}

impl<A: Copy + Eq + Hash> Input<A> {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            pressed_once: HashSet::new(),
            keymap: HashMap::new(),
            pressed_actions: HashSet::new(),
            pressed_actions_once: HashSet::new(),
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn map_key(&mut self, key: KeyCode, action: A) {
        self.keymap.insert(key, action);
    }

    pub fn clear_keymap(&mut self) {
        self.keymap.clear();
        self.pressed_actions.clear();
        self.pressed_actions_once.clear();
    }

    pub fn handle_event<T>(&mut self, event: &Event<T>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    let key_code = match event.physical_key {
                        PhysicalKey::Code(code) => code,
                        _ => return,
                    };

                    match event.state {
                        ElementState::Pressed => {
                            self.pressed_keys.insert(key_code);
                            self.pressed_once.insert(key_code);

                            if let Some(&action) = self.keymap.get(&key_code) {
                                self.pressed_actions.insert(action);
                                self.pressed_actions_once.insert(action);
                            }
                        }
                        ElementState::Released => {
                            self.pressed_keys.remove(&key_code);

                            if let Some(&action) = self.keymap.get(&key_code) {
                                self.pressed_actions.remove(&action);
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.mouse_delta.0 += delta.0;
                    self.mouse_delta.1 += delta.1;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn key_held(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_once.contains(&key)
    }

    pub fn action_held(&self, action: A) -> bool {
        self.pressed_actions.contains(&action)
    }
    pub fn action_pressed(&self, action: A) -> bool {
        self.pressed_actions_once.contains(&action)
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn end_frame(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.pressed_once.clear();
        self.pressed_actions_once.clear();
    }
} 