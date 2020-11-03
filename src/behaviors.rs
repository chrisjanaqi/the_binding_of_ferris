use legion::{Entity, World};

/// A trait that modify the behavior of a tear.
///
/// Items should implement this behavior if they intend to change
/// how tears update / hit / destroy themselves
trait TearBehavior {
    fn on_hit(&mut self, _world: &mut World, _tear: Entity, _other: Entity) {}

    fn on_update(&mut self, _world: &mut World, _tear: Entity) {}

    fn on_destroy(&mut self, _world: &mut World, _tear: Entity) {}
}

trait PlayerBehavior {
    fn on_fire(&mut self, _world: &mut World, _player: Entity) {}

    fn on_damage(&mut self, _world: &mut World, _player: Entity) {}

    fn on_update(&mut self, _world: &mut World, _player: Entity) {}
}
