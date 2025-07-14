use std::collections::HashMap;
use rapier3d::prelude::*;
use hecs::World;
use glam::{Vec3, Quat};
use crate::ecs::Transform;

pub struct Physics {
    rigid_bodies: RigidBodySet,
    colliders: ColliderSet,
    integration_params: IntegrationParameters,
    pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    gravity: Vector<f32>,
    entity_to_rb: HashMap<hecs::Entity, RigidBodyHandle>,
}

impl Physics {
    pub fn new() -> Self {
        let rigid_bodies = RigidBodySet::new();
        let colliders = ColliderSet::new();
        let integration_params = IntegrationParameters::default();
        let pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhaseMultiSap::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joints = ImpulseJointSet::new();
        let multibody_joints = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let gravity = vector![0.0, -9.81, 0.0];
        Self {
            rigid_bodies,
            colliders,
            integration_params,
            pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joints,
            multibody_joints,
            ccd_solver,
            query_pipeline,
            gravity,
            entity_to_rb: HashMap::new(),
        }
    }

    pub fn step(&mut self, dt: f32, world: &mut World) {
        let physics_hooks = ();
        let event_handler = ();
        self.integration_params.dt = dt;
        self.pipeline.step(
            &self.gravity,
            &self.integration_params,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        for (&ent, &handle) in &self.entity_to_rb {
            if let Ok(mut tr) = world.get::<&mut Transform>(ent) {
                let rb = self.rigid_bodies.get(handle).unwrap();
                let pos = rb.position();
                tr.translation = Vec3::new(pos.translation.x, pos.translation.y, pos.translation.z);
                let q = pos.rotation;
                tr.rotation = Quat::from_xyzw(q.i, q.j, q.k, q.w);
            }
        }
    }

    pub fn add_rigid_body(&mut self, ent: hecs::Entity, rb: RigidBody, collider: Collider) {
        let rb_handle = self.rigid_bodies.insert(rb);
        self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        self.entity_to_rb.insert(ent, rb_handle);
    }
} 