use crate::player::*;
use bevy::prelude::*;

pub trait Attribute: Copy + Send + Sync + 'static {}
impl<T: Copy + Send + Sync + 'static> Attribute for T {}

#[derive(Debug, Clone, Copy)]
pub struct Health(i32);

#[derive(Debug, Clone, Copy)]
pub struct Damage(f32);

#[derive(Debug, Clone, Copy)]
pub struct AttackRate(f32);

#[derive(Debug, Clone, Copy)]
pub struct Range(f32);

#[derive(Debug, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Debug, Clone, Copy)]
pub struct MovementSpeed(f32);


/// Event that inform of a change in a player's attribute
pub struct AttributeChange<A: Attribute> {
    pub player: Entity,
    pub attribute: A,
}

pub struct IsaacAttributes;

#[derive(Bundle)]
pub struct PlayerAttributes {
    pub health: Health,
    pub damage: Damage,
    pub attack_rate: AttackRate,
    pub range: Range,
    pub projectile_speed: ProjectileSpeed,
    pub movement_speed: MovementSpeed,
}

impl IsaacAttributes {
    fn update<A: Attribute>(
        mut event_reader: Local<EventReader<AttributeChange<A>>>,
        events: Res<Events<AttributeChange<A>>>,
        mut query: Query<With<Player, &mut A>>,
    ) {
        if let Some(&AttributeChange { player, attribute }) = event_reader.latest(&events) {
            if let Err(e) = query.set(player, attribute) {
                println!("Cannot change attribute for player {:?}: {:?}", player, e);
            }
        }
    }

    fn add_attribute<A: Attribute>(&self, app: &mut AppBuilder) -> &Self {
        app.add_event::<AttributeChange<A>>()
            .add_system(Self::update::<A>.system());
        self
    }
}

impl Plugin for IsaacAttributes {
    fn build(&self, app: &mut AppBuilder) {
        self.add_attribute::<Health>(app)
            .add_attribute::<Damage>(app)
            .add_attribute::<AttackRate>(app)
            .add_attribute::<Range>(app)
            .add_attribute::<ProjectileSpeed>(app)
            .add_attribute::<MovementSpeed>(app);
    }
}
