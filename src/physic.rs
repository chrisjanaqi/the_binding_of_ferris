use bevy::{core::FixedTimestep, prelude::*};

#[derive(Debug, Default)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Default)]
pub struct Movement {
    pub direction: Option<Vec2>,
    pub speed: f32,
    pub acceleration: f32,
    pub damping: f32,
}

pub struct IsaacPhysic;

impl IsaacPhysic {
    const FIXED_TIMESTEP: f64 = 0.01;

    fn moving(time: Res<Time>, mut query: Query<(&Movement, &mut Velocity)>) {
        let dt = time.delta_seconds();
        for (movement, mut velocity) in query.iter_mut() {
            let is_moving = movement.direction.is_some();
            let target = movement.speed * movement.direction.unwrap_or_default();
            let current = velocity.0;
            let error = target - current;
            let norm = error.length();
            if norm > f32::EPSILON {
                let coeff = if is_moving {
                    movement.acceleration
                } else {
                    movement.damping
                };
                velocity.0 = if norm <= coeff * dt {
                    target
                } else {
                    current + coeff * dt / norm * error
                };
            }
        }
    }

    fn physics(mut query: Query<(Option<&Velocity>, &mut Transform)>) {
        let dt = Self::FIXED_TIMESTEP as f32;
        for (velocity, mut transform) in query.iter_mut() {
            if let Some(Velocity(speed)) = velocity {
                transform.translation += speed.extend(0.0) * dt;
            }
        }
    }
}

impl Plugin for IsaacPhysic {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::moving.system()).add_stage_after(
            stage::UPDATE,
            "fixed_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(Self::FIXED_TIMESTEP))
                .with_system(Self::physics.system()),
        );
    }
}
