extern crate ray_tracer;

#[cfg(test)]
mod tests {
    use ray_tracer::utils::{Color, Point, Sphere, Scene, Vector3D, Ray};
    #[test]
    fn intersection() {
        let ray = Ray {
            origin: Point {x: 0., y: 0., z: 0.},
            direction: Vector3D {x: 0., y: 0., z: -1.},
        };
        let sphere = Sphere {
            center: Point {x: 0., y: 0., z: -5.},
            radius: 1.,
            color: Color {red: 0.4, green: 1.0, blue: 0.4},
        };
        assert!(sphere.is_intersect(ray));
    }
}
