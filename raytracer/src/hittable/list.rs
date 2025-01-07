use std::{ops::{Deref, Range}, sync::Arc};

use as_any::{AsAny, Downcast};

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
    
    pub fn get_geometries<T: Downcast + Clone>(&self) -> Vec<T> {
        let mut geometries = Vec::new();
        for object in &self.objects {
            if let Some(geometry) = (***object).downcast_ref::<T>() {
                geometries.push(geometry.clone());
            }
        }
        geometries
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