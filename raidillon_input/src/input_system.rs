use raidillon_core::{EventBus, GameEvent, InputAction, System, AssetManager, Model, Material};
use crate::Input;
use hecs::World;

pub struct InputSystem {
    input: Input<InputAction>,
}

impl InputSystem {
    pub fn new() -> Self {
        let mut input = Input::<InputAction>::new();
        input.map_key(winit::keyboard::KeyCode::KeyW, InputAction::MoveForward);
        input.map_key(winit::keyboard::KeyCode::KeyS, InputAction::MoveBackward);
        input.map_key(winit::keyboard::KeyCode::KeyA, InputAction::MoveLeft);
        input.map_key(winit::keyboard::KeyCode::KeyD, InputAction::MoveRight);
        
        Self { input }
    }

    pub fn handle_event<T>(&mut self, event: &winit::event::Event<T>) {
        self.input.handle_event(event);
    }

    pub fn update(&mut self, event_bus: &mut EventBus, right_mouse_held: bool) {
        if right_mouse_held {
            if self.input.action_held(InputAction::MoveForward) {
                event_bus.emit(GameEvent::InputAction(InputAction::MoveForward));
            }
            if self.input.action_held(InputAction::MoveBackward) {
                event_bus.emit(GameEvent::InputAction(InputAction::MoveBackward));
            }
            if self.input.action_held(InputAction::MoveLeft) {
                event_bus.emit(GameEvent::InputAction(InputAction::MoveLeft));
            }
            if self.input.action_held(InputAction::MoveRight) {
                event_bus.emit(GameEvent::InputAction(InputAction::MoveRight));
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.input.end_frame();
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.input.mouse_delta()
    }
}

impl System for InputSystem {
    fn update(&mut self, _world: &mut World, _resources: &AssetManager<dyn Model, dyn Material>, _events: &mut EventBus, _dt: f32) {
        // Input processing is handled separately in the main loop
        // This system mainly generates events based on input state
    }

    fn handle_event(&mut self, _event: &GameEvent, _world: &mut World) {
        // InputSystem doesn't need to respond to events
        // It generates events based on input state
    }

    fn name(&self) -> &'static str {
        "InputSystem"
    }
} 