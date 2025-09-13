use crate::math::Vec3;


#[derive(Clone, Copy)]
pub struct Ray { pub o: Vec3, pub d: Vec3 }


#[derive(Clone, Copy)]
pub struct Aabb { pub min: Vec3, pub max: Vec3 }


#[derive(Clone, Copy)]
pub struct Hit { pub t: f32, pub p: Vec3, pub n: Vec3, pub albedo: Vec3 }


pub struct Light { pub pos: Vec3, pub color: Vec3, pub intensity: f32 }


pub struct Material { pub albedo: Vec3, pub ambient: Vec3 }


pub struct Scene {
pub cube: Aabb,
pub plane_y: f32,
pub render_plane: bool,
pub light: Light,
pub material: Material,
}