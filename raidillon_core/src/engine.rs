use hecs::World;
use crate::{
    Time, EventBus, GameEvent, SystemRegistry, 
    AssetManager, Model, Material, ModelId
};

pub struct Engine {
    pub world: World,
    pub systems: SystemRegistry,
    pub assets: AssetManager<dyn Model, dyn Material>,
    pub events: EventBus,
    pub time: Time,
}

impl Engine {
    pub fn new() -> Self {
        let systems = SystemRegistry::new();
        
        Self {
            world: World::new(),
            systems,
            assets: AssetManager::new(),
            events: EventBus::new(),
            time: Time::new(),
        }
    }

    pub fn add_system<S: crate::System + 'static>(&mut self, system: S) {
        self.systems.add_system(system);
    }

    pub fn update(&mut self) {
        self.time.tick();
        let dt = self.time.delta_seconds();
        
        // Update all systems
        self.systems.update_all(&mut self.world, &self.assets, &mut self.events, dt);
        
        // Process events
        self.events.process();
    }

    pub fn handle_window_event(&mut self, event: &GameEvent) {
        self.events.emit(event.clone());
        self.systems.handle_event_for_all(event, &mut self.world);
    }

    pub fn load_model(&mut self, path: &str) -> anyhow::Result<ModelId> {
        // This is a placeholder - in a real implementation, we'd need to 
        // coordinate with the render system to actually load the model
        // For now, just return a dummy ID
        Ok(ModelId(0))
    }

    pub fn spawn_entity_with_model(&mut self, model_id: ModelId) -> hecs::Entity {
        // This would need proper Transform and ModelHandle types
        // For now, return a placeholder entity
        self.world.spawn(())
    }

    pub fn delta_time(&self) -> f32 {
        self.time.delta_seconds()
    }

    pub fn emit_event(&mut self, event: GameEvent) {
        self.events.emit(event);
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn system_count(&self) -> usize {
        self.systems.system_count()
    }
} 