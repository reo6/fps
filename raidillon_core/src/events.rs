use std::any::TypeId;
use std::collections::HashMap;

/// Core event enumeration.
/// Generic over the `Action` type to keep the engine agnostic to concrete
/// game-specific input enumerations.
#[derive(Debug, Clone)]
pub enum GameEvent<A> {
    InputAction(A),
    CameraMove { position: glam::Vec3, front: glam::Vec3 },
    WindowResize { width: u32, height: u32 },
    EntitySpawned(hecs::Entity),
}

pub trait EventHandler<A>: 'static {
    fn handle(&mut self, event: &GameEvent<A>);
}

pub struct EventBus<A> {
    events: Vec<GameEvent<A>>,
    subscribers: HashMap<TypeId, Vec<Box<dyn EventHandler<A>>>>,
}

impl<A: 'static + Clone> EventBus<A> {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe<H: EventHandler<A> + 'static>(&mut self, handler: H) {
        self.subscribers
            .entry(TypeId::of::<H>())
            .or_default()
            .push(Box::new(handler));
    }

    pub fn emit(&mut self, event: GameEvent<A>) {
        self.events.push(event);
    }

    /// Process all queued events, dispatching them to every registered
    /// subscriber.
    pub fn process(&mut self) {
        let events = std::mem::take(&mut self.events);
        for ev in &events {
            for subs in self.subscribers.values_mut() {
                for h in subs {
                    h.handle(ev);
                }
            }
        }
    }

    pub fn drain(&mut self) -> Vec<GameEvent<A>> {
        std::mem::take(&mut self.events)
    }
} 