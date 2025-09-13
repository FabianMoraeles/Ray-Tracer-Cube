#[derive(Clone, Copy, Debug)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
impl Vec3 {
pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
pub fn add(self, o: Vec3) -> Vec3 { Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z) }
pub fn sub(self, o: Vec3) -> Vec3 { Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z) }
pub fn mul(self, s: f32) -> Vec3 { Vec3::new(self.x * s, self.y * s, self.z * s) }
pub fn dot(self, o: Vec3) -> f32 { self.x * o.x + self.y * o.y + self.z * o.z }
pub fn cross(self, o: Vec3) -> Vec3 {
Vec3::new(self.y*o.z - self.z*o.y, self.z*o.x - self.x*o.z, self.x*o.y - self.y*o.x)
}
pub fn length(self) -> f32 { self.dot(self).sqrt() }
pub fn norm(self) -> Vec3 { let l = self.length().max(1e-8); self.mul(1.0/l) }
pub fn clamp01(self) -> Vec3 {
Vec3::new(self.x.clamp(0.0,1.0), self.y.clamp(0.0,1.0), self.z.clamp(0.0,1.0))
}
}