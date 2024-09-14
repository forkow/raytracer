use std::f32::consts::{PI, TAU};

use na::Vector3;

use super::{Hit, HitInfo};
use crate::world::{material::Material, ray::Ray};

#[derive(Debug)]
pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (-half_b - sqrtd) * a.recip();
        if t < t_min || t_max < t {
            t = (-half_b + sqrtd) * a.recip();
            if t < t_min || t_max < t {
                return None;
            }
        }

        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        let th = (-point.y).acos();
        let phi = fast_math::atan2(-point.z, point.x) + PI;

        let u = phi / TAU;
        let v = th / PI;

        Some(HitInfo {
            t,
            position: point,
            normal,
            u,
            v,
            material: &self.material,
        })
    }
}
