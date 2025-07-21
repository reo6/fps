use glam::Vec3;
use hecs::Entity;

#[derive(Debug, Clone)]
pub enum GameEvent {
    InputAction(InputAction),
    CameraMove { position: Vec3, front: Vec3 },
    WindowResize { width: u32, height: u32 },
    EntitySpawned(Entity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

pub trait EventHandler {
    fn handle(&mut self, event: &GameEvent);
}

pub struct EventBus {
    events: Vec<GameEvent>,
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            handlers: Vec::new(),
        }
    }

    pub fn emit(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn subscribe<H: EventHandler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }

    pub fn process(&mut self) {
        for event in self.events.drain(..) {
            for handler in &mut self.handlers {
                handler.handle(&event);
            }
        }
    }

    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
} 