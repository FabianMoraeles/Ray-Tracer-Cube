use crate::math::Vec3;
use crate::types::{Ray, Aabb};

pub fn box_center(b: Aabb) -> Vec3 {
    Vec3::new(0.5*(b.min.x + b.max.x), 0.5*(b.min.y + b.max.y), 0.5*(b.min.z + b.max.z))
}

pub fn intersect_aabb(ray: Ray, box_: Aabb, t_min: f32, t_max: f32) -> Option<(f32, Vec3)> {
    let mut t0 = t_min;
    let mut t1 = t_max;
    let mut update = |ro: f32, rd: f32, minv: f32, maxv: f32| {
        let rd = if rd.abs() < 1e-8 { 1e-8_f32.copysign(rd) } else { rd };
        let inv = 1.0 / rd;
        let mut t_near = (minv - ro) * inv;
        let mut t_far  = (maxv - ro) * inv;
        if t_near > t_far { std::mem::swap(&mut t_near, &mut t_far); }
        t0 = t0.max(t_near);
        t1 = t1.min(t_far);
    };
    update(ray.o.x, ray.d.x, box_.min.x, box_.max.x);
    update(ray.o.y, ray.d.y, box_.min.y, box_.max.y);
    update(ray.o.z, ray.d.z, box_.min.z, box_.max.z);
    if t1 >= t0 && t1 > 0.0 {
        let t = if t0 > 0.0 { t0 } else { t1 };
        let p = Vec3::new(ray.o.x + ray.d.x*t, ray.o.y + ray.d.y*t, ray.o.z + ray.d.z*t);
        Some((t, p))
    } else { None }
}

pub fn aabb_normal(p: Vec3, b: Aabb) -> Vec3 {
    let eps = 1e-4;
    if (p.x - b.min.x).abs() < eps { return Vec3::new(-1.0, 0.0, 0.0); }
    if (p.x - b.max.x).abs() < eps { return Vec3::new( 1.0, 0.0, 0.0); }
    if (p.y - b.min.y).abs() < eps { return Vec3::new( 0.0,-1.0, 0.0); }
    if (p.y - b.max.y).abs() < eps { return Vec3::new( 0.0, 1.0, 0.0); }
    if (p.z - b.min.z).abs() < eps { return Vec3::new( 0.0, 0.0,-1.0); }
    if (p.z - b.max.z).abs() < eps { return Vec3::new( 0.0, 0.0, 1.0); }
    p.sub(box_center(b)).norm()
}

pub fn intersect_plane_y(ray: Ray, plane_y: f32, t_min: f32, t_max: f32) -> Option<(f32, Vec3)> {
    let denom = ray.d.y;
    if denom.abs() < 1e-8 { return None; }
    let t = (plane_y - ray.o.y) / denom;
    if t > t_min && t < t_max {
        let p = Vec3::new(ray.o.x + ray.d.x*t, ray.o.y + ray.d.y*t, ray.o.z + ray.d.z*t);
        Some((t, p))
    } else { None }
}