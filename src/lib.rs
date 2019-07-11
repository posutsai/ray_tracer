extern crate image;
pub mod utils {
    use image::{Pixel, GenericImage};

    #[derive(Clone)]
    pub struct Vector3D {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    impl Vector3D {
        fn a2b_vec(a: &Point, b: &Point) -> Vector3D {
            return Vector3D { x: b.x - a.x, y: b.y - a.y, z: b.z - a.z, };
        }
        fn length(&self) -> f64 {
            return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        }
        fn unit_vec(&self) -> Vector3D {
            let len = self.length();
            return Vector3D {
                x: self.x / len, y: self.y / len, z: self.z / len,
            };
        }
    }
    #[derive(Clone)]
    pub struct Color {
        pub red: f32,
        pub green: f32,
        pub blue: f32,
    }
    impl Color {
        pub fn to_rgba(&self) -> image::Rgba<u8> {
            return image::Rgba::from_channels(
                (self.red * 255.0) as u8,
                (self.green * 255.0) as u8,
                (self.blue * 255.0) as u8,
                255,
            );
        }
    }

    #[derive(Clone)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    pub struct Sphere {
        pub center: Point,
        pub radius: f64,
        pub color: Color,
    }
    impl Sphere {
        pub fn is_intersect(&self, ray: Ray) -> bool {
            let v = Vector3D::a2b_vec(&ray.origin, &self.center.clone());
            let cos_theta = dot_3d(&v.unit_vec(), &ray.direction.unit_vec());
            let d = v.length() * (1. - cos_theta.powi(2)).sqrt();
            if d > self.radius {
                return false;
            }
            else {
                return true;
            }
        }
        pub fn cal_intersect_distance(&self, ray: &Ray) -> (bool, Option<f64>) {
            // This function is responsible for computing whether the ray intersect the object or
            // not and the distance between origin of the ray and the intersection.
            let v = Vector3D::a2b_vec(&ray.origin, &self.center.clone());
            let cos_theta = dot_3d(&v.unit_vec(), &ray.direction.unit_vec());
            let d = v.length() * (1. - cos_theta.powi(2)).sqrt();
            if d > self.radius {
                return (false, None);
            }
            else {
                let intersect = v.length() * cos_theta - (self.radius.powi(2) - d.powi(2)).sqrt();
                return (true, Some(intersect));
            }
        }
    }

    pub struct Scene {
        pub width: u32,
        pub height: u32,
        // fov stands for "field of view"
        pub fov: f64,
        pub spheres: Vec<Sphere>,
        pub camera_pos: Point,
    }
    impl Scene {
        pub fn interact_spheres(&self, ray: Ray) -> Color {
            let sphere_iter = self.spheres.iter();
            let mut pixel = Color {red: 0., green: 0., blue: 0.};
            let mut min_dist = std::f64::MAX;
            let mut is_hit = false;
            for s in sphere_iter {
                let (is_intersect, dist) = s.cal_intersect_distance(&ray);
                if is_intersect == true && dist.unwrap() < min_dist {
                    is_hit = true;
                    pixel = s.color.clone();
                    min_dist = dist.unwrap();
                }
            }
            return pixel;
        }
    }

    pub struct Ray {
        pub origin: Point,
        pub direction: Vector3D,
    }
    impl Ray {
        pub fn prime_ray_on_sensor(x: u32, y: u32, scene: &Scene) -> Ray {
            // Geometry correction for sensor origin coordinate
            // linear scale to -1. ~ 1.
            let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
            let aspect_ratio = (scene.width as f64) / (scene.height as f64);
            let ctr = Point {
                x: (((x as f64 + 0.5) / scene.width as f64) * 2. - 1.) * aspect_ratio * fov_adjustment,
                y: (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment,
                z: -1.
            };
            return Ray {
                origin: scene.camera_pos.clone(),
                direction: Vector3D::a2b_vec(&scene.camera_pos, &ctr),
            };
        }
    }

    #[inline]
    fn dot_3d(vec_a: &Vector3D, vec_b: &Vector3D) -> f64 {
        vec_a.x * vec_b.x + vec_a.y * vec_b.y + vec_a.z * vec_b.z
    }
    #[inline]
    fn cross_3d(vec_a: Vector3D, vec_b: Vector3D) -> Vector3D {
        return Vector3D {
            x: vec_a.y * vec_b.z - vec_a.z * vec_b.y,
            y: vec_a.z * vec_b.x - vec_a.x * vec_b.z,
            z: vec_a.x * vec_b.y - vec_a.y * vec_b.x,
        };
    }
    }
