mod math;
mod types;
mod geom;
mod camera;
mod render;


use image::ImageBuffer;
use math::Vec3;
use types::{Aabb, Light, Material, Scene};
use camera::{deg2rad, orbit, Camera};
use geom::box_center;


fn main() {
// switches rápidos
let render_plane = true; // quita el piso si te estorba


// imagen
let width: u32 = 900;
let height: u32 = 600;
let aspect = width as f32 / height as f32;


// cámara
let fov = deg2rad(60.0);
let focal = 1.0 / (fov*0.5).tan();


// cubo
let cube = Aabb {
min: Vec3::new(-1.0, -1.0, -1.0).add(Vec3::new(0.0, 0.0, -3.0)),
max: Vec3::new( 1.0, 1.0, 1.0).add(Vec3::new(0.0, 0.0, -3.0)),
};
let plane_y = cube.min.y - 0.001;
let center = box_center(cube);


// órbita (ajusta a gusto)
let yaw = deg2rad(40.0);
let pitch = deg2rad(18.0);
let radius = 5.0;
let (eye, forward, right, up) = orbit(center, yaw, pitch, radius);


let cam = Camera { eye, forward, right, up, focal, aspect };


// luz/material (solo difuso + ambiente)
let light = Light { pos: Vec3::new(3.5, 3.0, -1.0), color: Vec3::new(2.0, 1.0, 1.0), intensity: 2.0 };
let material = Material { albedo: Vec3::new(4.5, 1.30, 0.95), ambient: Vec3::new(0.06, 0.06, 0.06) };


let scene = Scene { cube, plane_y, render_plane, light, material };


let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = render::render(width, height, &cam, &scene);
img.save("output.png").expect("No se pudo guardar output.png");
println!("Listo: output.png");
}