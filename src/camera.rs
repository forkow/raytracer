use na::Vector3;

use crate::{
    color::Color,
    world::{ray::Ray, World},
};

pub struct Camera {
    origin: Vector3<f32>,
    vertical: Vector3<f32>,
    horizontal: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, -viewport_height, 0.0);
        let lower_left_corner = origin -
            horizontal * 0.5 -
            vertical * 0.5 -
            Vector3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            vertical,
            horizontal,
            lower_left_corner,
        }
    }

    pub fn trace(&self, u: f32, v: f32, depth: u32, world: &World) -> Color {
        let cam_ray = Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical -
                self.origin,
        );

        cam_ray.trace(world, depth, Color::rgb(0.2, 0.5, 1.0))
    }
}
