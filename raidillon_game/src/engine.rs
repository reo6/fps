use hecs::{World, Entity};
use glium::Surface;
use raidillon_render::{RenderSystem, window::DisplayHandle, Camera};
use raidillon_input::{InputSystem, CameraSystem, Action};
use raidillon_core::{Time, AssetManager, EventBus, SystemRegistry};
use glam::{Vec3, Quat};

pub struct Engine {
    world: World,
    assets: AssetManager,
    events: EventBus<Action>,
    systems: SystemRegistry<Action>,
    render_system: RenderSystem,
    input_system: InputSystem,
    time: Time,
    camera_entity: Entity,
}

impl Engine {
    pub fn new(display: &DisplayHandle) -> anyhow::Result<Self> {
        let mut world = World::new();
        let mut assets = AssetManager::new();
        let events = EventBus::new();
        let mut systems = SystemRegistry::new();
        let render_system = RenderSystem::new(display.clone())?;
        let input_system = InputSystem::new();
        let time = Time::new();

        let tree_model = assets.load_model("resources/models/tree.gltf", render_system.display())?;
        let ground_model = assets.load_model("resources/models/plane.gltf", render_system.display())?;

        world.spawn((raidillon_ecs::Transform {
            translation: Vec3::new(0.0, -2.5, -5.0),
            rotation:    Quat::IDENTITY,
            scale:       Vec3::splat(0.01),
        }, tree_model));

        world.spawn((raidillon_ecs::Transform {
            translation: Vec3::new(0.0, -1.5, 0.0),
            rotation:    Quat::IDENTITY,
            scale:       Vec3::ONE,
        }, ground_model));

        let camera_entity = world.spawn((Camera {
            eye:    Vec3::new(0.0, 0.0, 2.0),
            center: Vec3::ZERO,
            up:     Vec3::Y,
            fovy:   60_f32.to_radians(),
            aspect: 1280.0 / 720.0,
            znear:  0.1,
            zfar:   100.0,
        },));

        systems.add_system(CameraSystem::new(camera_entity));

        Ok(Self {
            world,
            assets,
            events,
            systems,
            render_system,
            input_system,
            time,
            camera_entity,
        })
    }

    pub fn handle_event<T>(&mut self, event: &winit::event::Event<T>) {
        self.input_system.handle_event(event);
        if let winit::event::Event::WindowEvent { event, .. } = event {
            if let winit::event::WindowEvent::Resized(sz) = event {
                if let Ok(mut cam) = self.world.query_one_mut::<&mut Camera>(self.camera_entity) {
                    cam.aspect = sz.width as f32 / sz.height as f32;
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.time.tick();
        let dt = self.time.delta_seconds();
        self.input_system.update(&mut self.events);
        self.systems.update_all(&mut self.world, &self.assets, &mut self.events, dt);
        let _ = self.events.drain();
    }

    pub fn render_into<S: Surface>(&mut self, target: &mut S) {
        self.render_system.render_into(&self.world, &self.assets, target);
    }
} 