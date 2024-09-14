use na::{Reflection3, Unit};

use super::{Scatter, ScatterInfo};
use crate::{
    color::Color,
    math::random_vector3_in_unit_hemisphere,
    sampler::Sampler2D,
    world::{object::HitInfo, ray::Ray},
};

#[derive(Debug)]
pub struct Metallic {
    albedo: Sampler2D<Color>,
    roughness: f32,
}

impl Metallic {
    pub fn new(albedo: Sampler2D<Color>, roughness: f32) -> Self {
        Self { albedo, roughness }
    }
}

impl Scatter for Metallic {
    #[inline(always)]
    fn scatter(&self, ray: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let surface_normal = hit_info.normal +
            self.roughness * random_vector3_in_unit_hemisphere();
        let mut scatter_direction = ray.direction();

        Reflection3::new(Unit::new_normalize(surface_normal), 0.0)
            .reflect(&mut scatter_direction);

        let attenuation = self.albedo.sample(hit_info.u, hit_info.v);

        Some(ScatterInfo {
            attenuation,
            scattered_ray: Ray::new(hit_info.position, scatter_direction),
        })
    }
}
