use image::{ImageBuffer, Rgb};
use crate::math::Vec3;
use crate::types::*;
use crate::geom::{intersect_aabb, intersect_plane_y, aabb_normal};
use crate::camera::Camera;
use crate::texture::Texture;

fn sky(dir: Vec3) -> Vec3 {
    let t = 0.5*(dir.y + 1.0);
    let top = Vec3::new(0.5, 0.7, 1.0);
    let bot = Vec3::new(1.0, 1.0, 1.0);
    Vec3::new(bot.x*(1.0-t)+top.x*t, bot.y*(1.0-t)+top.y*t, bot.z*(1.0-t)+top.z*t)
}

fn cube_uv(p: Vec3, n: Vec3, b: Aabb) -> (f32, f32) {
    let dx = b.max.x - b.min.x;
    let dy = b.max.y - b.min.y;
    let dz = b.max.z - b.min.z;
    if n.x > 0.5 { let u = (p.z - b.min.z) / dz; let v = (p.y - b.min.y) / dy; return (u, v); }
    else if n.x < -0.5 { let u = (b.max.z - p.z) / dz; let v = (p.y - b.min.y) / dy; return (u, v); }
    else if n.y > 0.5 { let u = (p.x - b.min.x) / dx; let v = (b.max.z - p.z) / dz; return (u, v); }
    else if n.y < -0.5 { let u = (p.x - b.min.x) / dx; let v = (p.z - b.min.z) / dz; return (u, v); }
    else if n.z > 0.5 { let u = (b.max.x - p.x) / dx; let v = (p.y - b.min.y) / dy; return (u, v); }
    else { let u = (p.x - b.min.x) / dx; let v = (p.y - b.min.y) / dy; return (u, v); }
}

pub fn render(width: u32, height: u32, cam: &Camera, scene: &Scene, tex: Option<&Texture>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    let eps = 1e-3;

    for y in 0..height {
        for x in 0..width {
            let px = ((x as f32 + 0.5) / width as f32) * 2.0 - 1.0;
            let py = 1.0 - ((y as f32 + 0.5) / height as f32) * 2.0;

            let dir_world = cam.right.mul(px * cam.aspect)
                .add(cam.up.mul(py))
                .add(cam.forward.mul(cam.focal))
                .norm();

            let ray = Ray { o: cam.eye, d: dir_world };
            let mut color = sky(dir_world);

            let mut best_hit: Option<Hit> = None;
            if let Some((t, p)) = intersect_aabb(ray, scene.cube, 0.001, 1e9) {
                let n = aabb_normal(p, scene.cube);
                best_hit = Some(Hit { t, p, n, albedo: scene.material.albedo, kind: HitKind::Cube });
            }
            if scene.render_plane {
                if let Some((t, p)) = intersect_plane_y(ray, scene.plane_y, 0.001, 1e9) {
                    if best_hit.map(|h| t < h.t).unwrap_or(true) {
                        let s = 1.0;
                        let cx = (p.x / s).floor() as i32;
                        let cz = (p.z / s).floor() as i32;
                        let is_dark = ((cx + cz) & 1) == 0;
                        let albedo = if is_dark { Vec3::new(0.22, 0.22, 0.22) } else { Vec3::new(0.75, 0.75, 0.75) };
                        best_hit = Some(Hit { t, p, n: Vec3::new(0.0, 1.0, 0.0), albedo, kind: HitKind::Plane });
                    }
                }
            }

            if let Some(hit) = best_hit {
                let to_light = scene.light.pos.sub(hit.p);
                let dist_l = to_light.length();
                let l_dir = to_light.mul(1.0 / dist_l);

                let shadow_ray = Ray { o: hit.p.add(hit.n.mul(eps)), d: l_dir };
                let mut occluded = false;

                if let Some((t_s, _)) = intersect_aabb(shadow_ray, scene.cube, 0.001, dist_l - eps) {
                    if t_s < dist_l - eps { occluded = true; }
                }
                if scene.render_plane && !occluded {
                    if let Some((t_s, p_s)) = intersect_plane_y(shadow_ray, scene.plane_y + 0.0005, 0.001, dist_l - eps) {
                        if t_s < dist_l - eps && p_s.y <= scene.light.pos.y { occluded = true; }
                    }
                }

                let n_dot_l = if occluded { 0.0 } else { hit.n.dot(l_dir).max(0.0) };
                let attenuation = (1.0 / (dist_l*dist_l)).min(1.0);

                let mut base = hit.albedo;
                if let (Some(tex), HitKind::Cube) = (tex, hit.kind) {
                    let (u, v) = cube_uv(hit.p, hit.n, scene.cube);
                    base = tex.sample(u, v); // solo textura
                }

                let diffuse = base.mul(n_dot_l * scene.light.intensity * attenuation);
                let lambert = diffuse.mulv(scene.light.color);
                let ambient_term = scene.material.ambient.mulv(base); // ambient * textura
                let final_rgb = ambient_term.add(lambert).clamp01();

                color = Vec3::new(
                    final_rgb.x.powf(1.0/2.2),
                    final_rgb.y.powf(1.0/2.2),
                    final_rgb.z.powf(1.0/2.2),
                );
            }

            img.put_pixel(
                x, y,
                Rgb([
                    (color.x * 255.0).round().clamp(0.0,255.0) as u8,
                    (color.y * 255.0).round().clamp(0.0,255.0) as u8,
                    (color.z * 255.0).round().clamp(0.0,255.0) as u8,
                ])
            );
        }
    }

    img
}