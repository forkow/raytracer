pub mod material;
pub mod object;
pub mod ray;

use self::{
    object::{Hit, HitInfo, Object},
    ray::Ray,
};

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }
}

impl Hit for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let mut closest_so_far = t_max;
        let mut hit_info = None;

        for object in self.objects.iter() {
            if let Some(info) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }

        hit_info
    }
}
