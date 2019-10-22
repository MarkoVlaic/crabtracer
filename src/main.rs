mod vec3;
mod ray;
mod renderer;
mod camera;
mod materials;
mod hitables;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitables::{ Sphere, Hitable, HitableList };
use crate::renderer::Renderer;
use crate::camera::Camera;

use crate::materials::{ Lambertian, Metal, Dielectric };

use rand::Rng;
use spinner::SpinnerBuilder;


fn color(r: &Ray, world: &impl Hitable, depth: u8) -> Vec3 {

    if let Some(record) = world.hit(&r, 0.001, 100000.0) {
        //println!("Hit {:?}", record.p);
        /*let N = Vec3::unit_vector(&record.normal);
        let target = record.p.clone() + record.normal.clone() + random_in_unit_sphere();
        return 0.5 * color( &Ray::new(&record.p, &(target - record.p.clone())), world);*/

        if depth >= 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some( (attenuation, scattered) ) = record.material.scatter(r, &record) {
            return color(&scattered, world, depth+1).mul(&attenuation);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {

    let width = 400;
    let height = 200;

    let antialiasing_iters = 100;

    let mut renderer = Renderer::new(width, height);

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5))),
    ]);

    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    let spinner = SpinnerBuilder::new("Creating your image...".into()).start();

    for y in 0..height {
        for x in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            spinner.update(format!("Processing {}/{}", y*width + x + 1, width*height));
            for _ in 0..antialiasing_iters {
                let f1: f32 = rng.gen();
                let f2: f32 = rng.gen();

                let u = (x as f32 + f1)/ width as f32;
                let v = (y as f32 + f2)/ height as f32;
                let r = cam.get_ray(u, v);

                col = col + color(&r, &world, 0).sqrt() * 255.0;
            }

            col = col.clone()/(antialiasing_iters as f32);

            renderer.draw(x, y, &col);
        }
    }

    spinner.message("Saving image".into());
    renderer.render("./out/eigth.png");
    spinner.message("Done".into());
    spinner.close();
}