pub mod behaviors;
pub mod camera;
pub mod inputs;
pub mod player;

pub use camera::*;
pub use inputs::*;
pub use player::*;
// pub struct TearMesh(pub Mesh);

pub struct DeltaTime(pub f32);
