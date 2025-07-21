use hecs::World;
use crate::assets::{AssetManager, Model, Material};
use crate::events::{EventBus, GameEvent};

pub trait System {
    fn update(&mut self, world: &mut World, resources: &AssetManager<dyn Model, dyn Material>, events: &mut EventBus, dt: f32);
    fn handle_event(&mut self, event: &GameEvent, world: &mut World);
    fn name(&self) -> &'static str;
}

pub struct SystemRegistry {
    systems: Vec<Box<dyn System>>,
}

impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn update_all(&mut self, world: &mut World, resources: &AssetManager<dyn Model, dyn Material>, events: &mut EventBus, dt: f32) {
        for system in &mut self.systems {
            system.update(world, resources, events, dt);
        }
    }

    pub fn handle_event_for_all(&mut self, event: &GameEvent, world: &mut World) {
        for system in &mut self.systems {
            system.handle_event(event, world);
        }
    }

    pub fn system_count(&self) -> usize {
        self.systems.len()
    }

    pub fn clear(&mut self) {
        self.systems.clear();
    }
} 