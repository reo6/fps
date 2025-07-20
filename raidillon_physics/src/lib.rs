use rapier3d::prelude::*;
use raidillon_ecs::Transform;
use glam::{Quat, Vec3};
use ::nalgebra::{UnitQuaternion, Quaternion};
use rapier3d::geometry::{DefaultBroadPhase, NarrowPhase};
use rapier3d::prelude::QueryPipeline;

#[derive(Copy, Clone)]
pub struct RigidBodyComponent(pub RigidBodyHandle);

#[derive(Copy, Clone, Debug)]
pub enum BodyKind {
    Static,
    Dynamic,
    Kinematic,
}

pub struct Physics {
    pub rigid_body_set: RigidBodySet,
    pub collider_set:   ColliderSet,
    gravity:           Vector<f32>,
    integration_parameters: IntegrationParameters,
    island_manager:    IslandManager,
    broad_phase:       DefaultBroadPhase,
    narrow_phase:      NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    query_pipeline:    QueryPipeline,
    ccd_solver:        CCDSolver,
    physics_pipeline:  PhysicsPipeline,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set:   ColliderSet::new(),
            gravity:        vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase:    DefaultBroadPhase::new(),
            narrow_phase:   NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            query_pipeline: QueryPipeline::new(),
            ccd_solver:     CCDSolver::new(),
            physics_pipeline: PhysicsPipeline::new(),
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.integration_parameters.dt = dt;
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(), // physics hooks
            &(), // event handler
        );
    }

    pub fn add_rigid_body(&mut self, kind: BodyKind, transform: Transform, collider: Collider) -> RigidBodyHandle {
        let body_type = match kind {
            BodyKind::Static => RigidBodyType::Fixed,
            BodyKind::Dynamic => RigidBodyType::Dynamic,
            BodyKind::Kinematic => RigidBodyType::KinematicPositionBased,
        };

        let rb = RigidBodyBuilder::new(body_type)
            .translation(vector![transform.translation.x, transform.translation.y, transform.translation.z])
            .build();

        let rb_handle = self.rigid_body_set.insert(rb);

        // Attach collider to rigid body
        self.collider_set.insert_with_parent(collider, rb_handle, &mut self.rigid_body_set);

        rb_handle
    }

    pub fn get_rigid_body(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.rigid_body_set.get(handle)
    }

    pub fn get_rigid_body_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rigid_body_set.get_mut(handle)
    }

    fn quat_to_na(quat: Quat) -> UnitQuaternion<f32> {
        UnitQuaternion::from_quaternion(Quaternion::new(quat.w, quat.x, quat.y, quat.z))
    }

    pub fn rapier_translation_to_glam(v: &Vector<f32>) -> Vec3 {
        Vec3::new(v.x, v.y, v.z)
    }

    pub fn rapier_rotation_to_glam(r: &UnitQuaternion<f32>) -> Quat {
        Quat::from_xyzw(r.i, r.j, r.k, r.w)
    }
} 