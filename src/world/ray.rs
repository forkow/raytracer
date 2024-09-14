use na::Vector3;

use super::object::Hit;
use crate::{color::Color, world::material::Scatter};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    #[inline(always)]
    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    #[inline(always)]
    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }

    pub fn trace(&self, obj: &impl Hit, depth: u32, sky_color: Color) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some(hit_info) = obj.hit(self, 0.001, f32::INFINITY) {
            if let Some(scatter_info) =
                hit_info.material.scatter(&self, &hit_info)
            {
                return scatter_info.attenuation *
                    scatter_info.scattered_ray.trace(
                        obj,
                        depth - 1,
                        sky_color,
                    );
            } else {
                return Color::black();
            }
        } else {
            let a = 0.5 * (self.direction.y + 1.0);
            (1.0 - a) * Color::gray(1.0) + a * sky_color
        }
    }
}
