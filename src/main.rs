mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod renderer;
mod camera;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hitable_list::HitableList;
use crate::hitable::{ Hitable, HitRecord };
use crate::renderer::Renderer;
use crate::camera::Camera;

use rand::Rng;

fn color(r: &Ray, world: &impl Hitable) -> Vec3 {

    if let Some(record) = world.hit(&r, 0.0, 100000.0) {
        let N = Vec3::unit_vector(&record.normal);
        return (N + Vec3::new(1.0, 1.0, 1.0))*0.5;
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let width = 200;
    let height = 100;

    let antialiasing_iters = 100;

    let mut renderer = Renderer::new(width, height);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.25),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -3.0), 0.25)
    ]);

    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..antialiasing_iters {
                let f1: f32 = rng.gen();
                let f2: f32 = rng.gen();

                let u = (x as f32 + f1)/ width as f32;
                let v = (y as f32 + f2)/ height as f32;
                let r = cam.get_ray(u, v);

                col = col + color(&r, &world) * 255.0;
            }

            col = col.clone()/(antialiasing_iters as f32);

            renderer.draw(x, y, &col);
        }
    }

    renderer.render("./out/fifth.png");

}