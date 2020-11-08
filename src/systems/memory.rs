use crate::components::*;
use crate::model::DeltaTime;

use legion::systems::CommandBuffer;
use legion::*;

#[system(for_each)]
pub fn lifetime(
    entity: &Entity,
    ecb: &mut CommandBuffer,
    life: &mut Lifetime,
    #[resource] delta: &DeltaTime,
) {
    let DeltaTime(dt) = delta;
    life.0 -= dt;
    if life.0 <= 0.0 {
        ecb.remove(*entity);
    }
}
