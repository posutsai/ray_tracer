extern crate ray_tracer;

#[cfg(test)]
mod tests {
    use ray_tracer::utils::{Color, Point, Sphere, Scene, Vector3D, Ray};
    #[test]
    fn intersection() {
        let ray = ray_tracer::utils::Ray {
            origin: Point {x: 0., y: 0., z: 0.},
            direction: Vector3D {x: 0., y: 0., z: -1.},
        };
        assert_eq!(2, 2);
    }
}
