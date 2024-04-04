use std::ops::Range;

use crate::{ray::Ray, Float};

use super::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut t_range = t_range;

        for object in &self.objects {
            if let Some(record) = object.hit(ray, t_range.clone()) {
                t_range.end = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}