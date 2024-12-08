use std::{ops::Range, sync::Arc};

use crate::{ray::Ray, Float};

use super::{aabb::AABB, bvh::BVH, HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Arc<Box<dyn Hittable>>>,
    bbox: Option<AABB>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: None,
        }
    }

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        if let Some(bbox) = self.bbox {
            self.bbox = Some(AABB::merge(bbox, object.bounding_box()));
        } else {
            self.bbox = Some(object.bounding_box());
        }
        self.objects.push(Arc::new(object));
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

    fn bounding_box(&self) -> AABB {
        self.bbox.unwrap_or_default()
    }
}

impl BVH {
    pub fn new(list: &HittableList) -> Self {
        let mut objects = list.objects.clone();
        Self::new_with_mut_slice(&mut objects)
    }
}