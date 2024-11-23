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
use std::iter;
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
    #[arg(long)]
    samples: usize,
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

    draw(
        &scene,
        pixels.as_mut_slice(),
        args.width,
        height,
        args.samples,
    );

    color::write_ppm(args.width, height, pixels.as_slice(), &mut io::stdout())?;

    Ok(())
}

fn draw(scene: &Scene, pixels: &mut [Color], width: usize, height: usize, samples: usize) {
    let mut rec = rerun::RecordingStreamBuilder::new("camera")
        .save("/home/notroot/tmp/camera.rrd")
        .unwrap();

    scene.rec(&mut rec);

    for y in 0..height {
        eprintln!("processing row {y}");
        for x in 0..width {
            let color = iter::repeat_with(|| {
                let x = OsRng.gen_range(x as f64..x as f64 + 1.0);
                let y = OsRng.gen_range(y as f64..y as f64 + 1.0);
                let clip = Point2::new(
                    ((x / width as f64) * 2.0) - 1.0,
                    ((y / height as f64) * 2.0) - 1.0,
                );
                let ray = scene.camera.cast_ray(clip, &mut rec);
                ray_color(scene, ray, &mut rec)
            })
            .take(samples)
            .reduce(|acc, e| acc + e)
            .unwrap()
                / samples as f64;

            pixels[x + (y * width)] = color;
        }
    }
}

fn ray_color(scene: &Scene, ray: Ray, rec: &mut rerun::RecordingStream) -> Color {
    match closest_hit(scene, ray) {
        Some(hit) => match hit {
            Record::Light { color, .. } => color,
            Record::Object {
                material,
                normal,
                t,
            } => match material {
                Material::Diffuse { color, adsorption } => {
                    rec.log(
                        "normals",
                        &rerun::Arrows3D::from_vectors([[
                            normal.x as f32,
                            normal.y as f32,
                            normal.z as f32,
                        ]]),
                    )
                    .unwrap();
                    let diffused = Ray {
                        origin: ray.at(t),
                        direction: normal + random_unit_vec(),
                    };
                    rec.log(
                        "diffused",
                        &rerun::Arrows3D::from_vectors([[
                            diffused.direction.x as f32,
                            diffused.direction.y as f32,
                            diffused.direction.z as f32,
                        ]])
                        .with_origins([[
                            diffused.origin.x as f32,
                            diffused.origin.y as f32,
                            diffused.origin.z as f32,
                        ]]),
                    )
                    .unwrap();
                    ray_color(scene, diffused, rec) * color * adsorption
                }
                _ => color::BLACK,
            },
        },
        None => {
            let ud = ray.direction.normalize();
            let blue = Color::new(0.5, 0.7, 1.0, 0.0);
            let a = 0.5 * (ud.y + 1.0);
            (1.0 - a) * color::WHITE + a * blue
        }
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
