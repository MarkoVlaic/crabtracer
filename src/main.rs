mod vec3;
mod ray;
mod renderer;
mod camera;
mod materials;
mod hitables;
mod cmd_args;

use crate::vec3::Vec3;
use crate::renderer::Renderer;
use crate::camera::Camera;

use crate::hitables::{ HitableList, Sphere };
use crate::materials::{ Lambertian, Metal, Dielectric };

use crate::cmd_args::CmdArgs;

use spinner::SpinnerBuilder;
use std::process::exit;

fn main() {

    let args = CmdArgs::new();

    if let Err(e) = args {
        eprintln!("{}", e);
        exit(1);
    }

    let args = args.unwrap();

    let width = args.width;
    let height = args.height;

    let mut renderer = Renderer::new(width, height, args.sample_rate);

    //let cam = Camera::new(&Vec3::new(-2.0, 2.0, 1.0), &Vec3::new(0.0, 0.0, -1.0), &Vec3::new(0.0, 1.0, 0.0), 90.0, width as f32 / height as f32);
    
    let cam = Camera::new(&Vec3::new(0.0, 0.5, 0.0), &Vec3::new(0.0, 0.0, -1.0), &Vec3::new(0.0, 1.0, 0.0), 90.0, width as f32 / height as f32);

    /*let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Dielectric::new(1.4))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.5, 0.3, 0.3), 0.1))),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.3, 0.6, 0.3)))),
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.1, 0.7, 0.5)))),
    ]);*/

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.1, 0.7, 0.5)))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.1, 0.2, 0.3), 0.3))),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.4, 0.2, 0.2), 0.1))),
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.3), 0.5, Box::new(Dielectric::new(1.3))),
        Sphere::new_boxed(Vec3::new(-0.5, 0.0, -1.4), 0.2, Box::new(Lambertian::new(Vec3::new(0.1, 0.6, 0.1)))),
        Sphere::new_boxed(Vec3::new(0.3, 0.0, -0.8), 0.2, Box::new(Lambertian::new(Vec3::new(0.3, 0.3, 0.3))))
    ]);

    let spinner = SpinnerBuilder::new("Creating your image...".into()).start();
    
    renderer.render_image(&cam, &world, |x, y| {
        spinner.update(format!("Processing {}/{}", y*width + x + 1, width*height));
    });

    spinner.message("Saving image".into());
    renderer.output_image(args.filename);
    spinner.message("Done".into());
    spinner.close();
}