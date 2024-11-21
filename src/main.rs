use clap::Parser;
use color::Color;
use hit::{Hit, Record};
use material::Material;
use nalgebra::{Point2, Vector3};
use rand::rngs::OsRng;
use rand::Rng;
use ray::Ray;
use scene::Scene;
use std::error::Error;
use std::io::{self, Read};
use std::{fs::File, path::PathBuf};

mod camera;
mod color;
mod hit;
mod light;
mod material;
mod mesh;
mod ray;
mod scene;
mod serialize;

#[derive(clap::Parser)]
struct Args {
    #[arg(long)]
    width: usize,
    #[arg(long)]
    aspect_ratio: f64,
    #[arg(long)]
    scene: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let height = (args.width as f64 / args.aspect_ratio) as usize;

    let mut file = File::open(args.scene.as_path())?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    let deserialized_scene: crate::serialize::Scene = serde_json::from_str(string.as_str())?;
    let scene = Scene::from_deserialized(&deserialized_scene);

    let mut pixels = vec![crate::color::BLACK; args.width * height];

    draw(&scene, pixels.as_mut_slice(), args.width, height);

    color::write_ppm(args.width, height, pixels.as_slice(), &mut io::stdout())?;

    Ok(())
}

fn draw(scene: &Scene, pixels: &mut [Color], width: usize, height: usize) {
    let mut rec = rerun::RecordingStreamBuilder::new("camera")
        .save("/home/notroot/tmp/camera.rrd")
        .unwrap();

    scene.rec(&mut rec);

    for y in 0..height {
        for x in 0..width {
            let clip = Point2::new(
                ((x as f64 / width as f64) * 2.0) - 1.0,
                ((y as f64 / height as f64) * 2.0) - 1.0,
            );
            let ray = scene.camera.cast_ray(clip, &mut rec);

            pixels[x + (y * width)] = ray_color(scene, ray);
        }
    }
}

fn ray_color(scene: &Scene, ray: Ray) -> Color {
    match closest_hit(scene, ray) {
        Some(hit) => match hit {
            Record::Light { color, .. } => color,
            Record::Object {
                material,
                normal,
                t,
            } => match material {
                Material::Diffuse { color, adsorption } => color,
                _ => color::BLACK,
            },
        },
        None => scene.background,
    }
}

fn closest_hit(scene: &Scene, ray: Ray) -> Option<Record> {
    let mut interval = f64::EPSILON..f64::INFINITY;
    let mut closest = None;

    for light in &scene.lights {
        match light.hit(ray, interval.clone()) {
            Some(hit) => {
                closest = Some(hit);
                interval.end = hit.t()
            }
            None => continue,
        }
    }

    for mesh in &scene.meshes {
        match mesh.hit(ray, interval.clone()) {
            Some(hit) => {
                closest = Some(hit);
                interval.end = hit.t();
            }
            None => continue,
        }
    }

    closest
}

fn random_unit_vec() -> Vector3<f64> {
    loop {
        let p = Vector3::new(
            OsRng.gen_range(-1.0..1.0),
            OsRng.gen_range(-1.0..1.0),
            OsRng.gen_range(-1.0..1.0),
        );
        if p.magnitude_squared() < 1.0 {
            break p.normalize();
        }
    }
}
