extern crate image;

use image::GenericImage;
use std::fs::File;
use std::path::Path;
use std::mem;
use ray_tracer::utils::{Color, Point, Sphere, Scene, Vector3D, Ray, Plane, LightSrc};

fn render(scene: &Scene) -> image::DynamicImage {
    let mut img = image::DynamicImage::new_rgb8(scene.width, scene.height);
    // let p: image::Rgba<u8> = image::Rgba::from_channels(0, 122, 0, 0);
    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = Ray::prime_ray_on_sensor(x, y, scene);
            let p = scene.interact_spheres(ray);
            img.put_pixel(x, y, p.to_rgba());
            // if scene.sphere.is_intersect(ray) {
            // }
        }
    }
    return img;
}



fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![
            Box::new(Sphere {
                center: Point {x: 0., y: 0.5, z: -4.},
                radius: 1.,
                color: Color {red: 0.4, green: 1.0, blue: 0.4},
                albedo: 0.55,
            }),
            Box::new(Sphere {
                center: Point {x: 1.5, y: 2., z: -5.},
                radius: 1.5,
                color: Color {red: 0.8, green: 0.2, blue: 0.7},
                albedo: 0.95,
            }),
            Box::new(Sphere {
                center: Point {x: -5.5, y: 3.7, z: -8.},
                radius: 3.,
                color: Color {red: 0.2, green: 1., blue: 0.7},
                albedo: 0.95,
            }),
            Box::new(Plane {
                normal: Vector3D {x: 0., y: 1.0, z: 0.},
                pt: Point {x: 0., y: -20., z: 0.},
                color: Color {red:0.1, green: 0.3, blue: 1.},
                albedo: 0.95,
            })

        ],
        lights: vec![
            LightSrc {
                direction: Vector3D {x: 10., y: 10., z: -1.},
                color: Color {red: 1., green: 1., blue: 1.},
                intensity: 1.,
            }
        ],
        camera_pos: Point {x: 0., y: 0., z: 0.},
    };
    let img: image::DynamicImage = render(&scene);
    img.save("out.png").unwrap();
}
