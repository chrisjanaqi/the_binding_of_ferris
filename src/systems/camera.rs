use crate::components::*;
use crate::model::Camera;
use legion::*;

#[system(for_each)]
pub fn world_to_camera(
    translation: &Translation,
    rotation: &Rotation,
    size: &Size,
    w2s: &mut WorldToScreen,
    #[resource] camera: &Camera,
) {
    let (sin, cos) = camera.rotation.0.sin_cos();
    w2s.s = size.0 * camera.zoom;
    w2s.r.0 = rotation.0.0 + camera.rotation.0;
    w2s.t.x = camera.zoom * cos * translation.0.x - sin * translation.0.y + camera.position.x;
    w2s.t.y = sin * translation.0.x + camera.zoom * cos * translation.0.y + camera.position.y;
}

#[system(for_each)]
pub fn camera_to_screen(transform: &mut WorldToScreen) {
    transform.t *= Camera::HEIGHT;
}
