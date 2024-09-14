pub mod metal;

use self::metal::Metallic;
use super::{object::HitInfo, ray::Ray};
use crate::color::Color;

#[derive(Debug)]
pub enum Material {
    Metallic(Metallic),
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo>;
}

pub struct ScatterInfo {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

impl Scatter for Material {
    #[inline(always)]
    fn scatter(&self, ray: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        match self {
            Material::Metallic(metal) => metal.scatter(ray, hit_info),
        }
    }
}
