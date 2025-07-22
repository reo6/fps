use hecs::World;

use crate::{AssetManager, EventBus};

/// A game/engine system that updates every frame.
pub trait System<A> {
    /// Update the system for the current frame.
    ///
    /// * `world`  – mutable ECS world
    /// * `assets` – read-only resource manager
    /// * `events` – event bus for publishing/consuming game events
    /// * `dt`     – time delta in seconds
    fn update(&mut self, world: &mut World, assets: &AssetManager, events: &mut EventBus<A>, dt: f32);
}

/// Stores and updates a collection of boxed systems.
pub struct SystemRegistry<A> {
    systems: Vec<Box<dyn System<A>>>,
}

impl<A> SystemRegistry<A> {
    pub fn new() -> Self { Self { systems: Vec::new() } }

    pub fn add_system<S: System<A> + 'static>(&mut self, sys: S) {
        self.systems.push(Box::new(sys));
    }

    pub fn update_all(&mut self, world: &mut World, assets: &AssetManager, events: &mut EventBus<A>, dt: f32) {
        for s in &mut self.systems {
            s.update(world, assets, events, dt);
        }
    }
} 