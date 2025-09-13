use crate::math::Vec3;

pub struct Texture {
    w: u32,
    h: u32,
    data: image::RgbImage,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        // deja el PNG en la raíz (junto a Cargo.toml) como "texture.png"
        let img = image::open(path).expect("No se encontró texture.png").to_rgb8();
        let (w, h) = img.dimensions();
        Self { w, h, data: img }
    }
    pub fn sample(&self, u: f32, v: f32) -> Vec3 {
        // repeat (envuelve); si quieres clamp, usa clamp01
        let mut uu = u - u.floor(); if uu < 0.0 { uu += 1.0; }
        let mut vv = v - v.floor(); if vv < 0.0 { vv += 1.0; }
        let x = (uu * (self.w as f32 - 1.0)).round() as u32;
        let y = ((1.0 - vv) * (self.h as f32 - 1.0)).round() as u32; // V hacia arriba
        let p = self.data.get_pixel(x, y);
        Vec3::new(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0)
    }
}