use nalgebra::{Vector3, vector};
use rapier3d::prelude::{
    BroadPhaseMultiSap, CCDSolver, ColliderSet, DefaultBroadPhase, EventHandler, ImpulseJointSet,
    IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsHooks,
    PhysicsPipeline, QueryPipeline, RigidBodySet,
};

use super::Engine;

pub(super) struct Physics {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    integreation_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    gravity: Vector3<f32>,
    physics_hooks: Box<dyn PhysicsHooks>,
    event_handler: Box<dyn EventHandler>,
}
impl<'a> Engine<'a> {
    pub fn add_physics(mut self) -> Self {
        self.physics = Some(Physics {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integreation_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            physics_hooks: Box::new(()),
            event_handler: Box::new(()),
            gravity: vector![0.0, -9.81, 0.0],
        });
        self
    }
    pub fn physics_step(&mut self) {
        if let Some(physics) = &mut self.physics {
            let rigid_body_set = &mut physics.rigid_body_set;
            let collider_set = &mut physics.collider_set;
            let integration_parameters = &mut physics.integreation_parameters;
            let island_manger = &mut physics.island_manager;
            let broad_phase = &mut physics.broad_phase;
            let narrow_phase = &mut physics.narrow_phase;
            let impulse_joint_set = &mut physics.impulse_joint_set;
            let multibody_joint_set = &mut physics.multibody_joint_set;
            let ccd_solver = &mut physics.ccd_solver;
            let query_pipeline = &mut physics.query_pipeline;
            let gravity = &physics.gravity;
            let physics_hooks = &*physics.physics_hooks;
            let event_handler = &*physics.event_handler;

            physics.physics_pipeline.step(
                gravity,
                integration_parameters,
                island_manger,
                broad_phase,
                narrow_phase,
                rigid_body_set,
                collider_set,
                impulse_joint_set,
                multibody_joint_set,
                ccd_solver,
                Some(query_pipeline),
                physics_hooks,
                event_handler,
            );
        }
    }
    pub fn sync_physics(&mut self) {
        if let Some(physics) = &mut self.physics {
            for element in &mut self.elements {
                if let Some(handle) = element.rigid_body_handle {
                    if let Some(rigid_body) = physics.rigid_body_set.get(handle) {
                        let position = rigid_body.position();
                        element.model = position.to_homogeneous();
                    }
                }
            }
        }
    }
}
