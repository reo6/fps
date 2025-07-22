use crate::{Input, Action};
use raidillon_core::{System, AssetManager};
use hecs::World;
use raidillon_core::EventBus;
use raidillon_core::GameEvent;

pub struct InputSystem {
    input: Input<Action>,
}

impl InputSystem {
    pub fn new() -> Self {
        let mut input = Input::<Action>::new();
        use winit::keyboard::KeyCode;
        input.map_key(KeyCode::KeyW, Action::MoveForward);
        input.map_key(KeyCode::KeyS, Action::MoveBackward);
        input.map_key(KeyCode::KeyA, Action::MoveLeft);
        input.map_key(KeyCode::KeyD, Action::MoveRight);
        Self { input }
    }

    pub fn handle_event<T>(&mut self, event: &winit::event::Event<T>) {
        self.input.handle_event(event);
    }

    pub fn update(&mut self, bus: &mut EventBus<Action>) {
        for action in [Action::MoveForward, Action::MoveBackward, Action::MoveLeft, Action::MoveRight] {
            if self.input.action_held(action) {
                bus.emit(GameEvent::InputAction(action));
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.input.end_frame();
    }
}

impl System<crate::Action> for InputSystem {
    fn update(&mut self, _world: &mut World, _assets: &AssetManager, events: &mut raidillon_core::EventBus<crate::Action>, _dt: f32) {
        self.update(events);
        self.end_frame();
    }
} 