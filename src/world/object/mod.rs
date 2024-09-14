pub mod sphere;

use na::Vector3;

use self::sphere::Sphere;
use super::{material::Material, ray::Ray};

#[derive(Debug)]
pub enum Object {
    Sphere(Sphere),
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>;
}

#[derive(Debug)]
pub struct HitInfo<'a> {
    pub t: f32,
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub u: f32,
    pub v: f32,
    pub material: &'a Material,
}

impl Hit for Object {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}
