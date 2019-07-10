extern crate image;

use image::{Pixel, GenericImage};
use std::fs::File;
use std::path::Path;
use ray_tracer::utils::{Color, Point, Sphere, Scene, Vector3D, Ray};

fn render(scene: &Scene) -> image::DynamicImage {
    let mut img = image::DynamicImage::new_rgb8(scene.width, scene.height);
    let p: image::Rgba<u8> = image::Rgba::from_channels(0, 122, 0, 0);
    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = Ray::prime_ray_on_sensor(x, y, scene);
            if scene.sphere.is_intersect(ray) {
                img.put_pixel(x, y, p);
                println!("{}", x);
            }
        }
    }
    img
}



fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {x: 0., y: 0., z: -5.},
            radius: 1.,
            color: Color {red: 0.4, green: 1.0, blue: 0.4},
        },
        camera_pos: Point {x: 0., y: 0., z: 0.},
    };
    let img: image::DynamicImage = render(&scene);
    img.save("out.png").unwrap();
}
