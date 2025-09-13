use crate::math::Vec3;

pub fn deg2rad(d: f32) -> f32 { d * std::f32::consts::PI / 180.0 }

pub struct Camera {
    pub eye: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub focal: f32,
    pub aspect: f32,
}

pub fn orbit(center: Vec3, yaw: f32, pitch: f32, radius: f32) -> (Vec3, Vec3, Vec3, Vec3) {
    let eye = center.add(Vec3::new(
        radius * pitch.cos() * yaw.cos(),
        radius * pitch.sin(),
        radius * pitch.cos() * yaw.sin(),
    ));
    let world_up = Vec3::new(0.0, 1.0, 0.0);
    let forward = center.sub(eye).norm();
    let right = forward.cross(world_up).norm();
    let up = right.cross(forward).norm();
    (eye, forward, right, up)
}