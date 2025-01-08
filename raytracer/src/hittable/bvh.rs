use std::{cmp::Ordering, ops::Range, sync::Arc};

use crate::{hittable::{HitRecord, Hittable}, ray::Ray, utils::random::random_range, Float};

use super::aabb::AABB;

pub struct BVH {
    root: Arc<Node>,
}

impl BVH {
    pub fn new(list: &Vec<Arc<Box<dyn Hittable>>>) -> Self {
        let mut objects = list.clone();
        Self::new_with_mut_slice(&mut objects)
    }

    fn new_with_mut_slice(objects: &mut [Arc<Box<dyn Hittable>>]) -> Self {
        Self {
            root: Node::new(objects)
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord> {
        self.root.hit(ray, t_range)
    }

    fn bounding_box(&self) -> AABB {
        self.root.bbox
    }
}

struct Node {
    left: Option<Arc<Node>>,
    right: Option<Arc<Node>>,
    hittable: Option<Arc<Box<dyn Hittable>>>,
    bbox: AABB,
}

impl Node {
    fn new(objects: &mut [Arc<Box<dyn Hittable>>]) -> Arc<Self> {
        let mut bbox = objects[0].bounding_box();
        for object in &objects[1..] {
            bbox = AABB::merge(bbox, object.bounding_box());
        }
        let axis = bbox.longest_axis();

        if objects.len() == 1 {
            let object = objects[0].clone();
            let bbox = object.bounding_box();
            Arc::new(Self {
                left: None,
                right: None,
                hittable: Some(object),
                bbox,
            })
        } else if objects.len() == 2 {
            let left = Node::new(&mut objects[0..1]);
            let right = Node::new(&mut objects[1..2]);
            let bbox = AABB::merge(left.bounding_box(), right.bounding_box());
            Arc::new(Self {
                left: Some(left),
                right: Some(right),
                hittable: None,
                bbox,
            })
        } else {
            let mut objects = objects.to_vec();
            objects.sort_by(|a, b| {
                AABB::compare(a.bounding_box(), b.bounding_box(), axis)
            });
            let mid = objects.len()/2;
            let left = Node::new(&mut objects[..mid]);
            let right = Node::new(&mut objects[mid..]);
            let bbox = AABB::merge(left.bounding_box(), right.bounding_box());
            Arc::new(Self {
                left: Some(left),
                right: Some(right),
                hittable: None,
                bbox,
            })
        }
    }
}

impl Hittable for Node {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord> {
        if !self.bbox.intersect(ray, t_range.clone()) {
            return None;
        }

        if let Some(object) = self.hittable.as_ref() {
            object.hit(ray, t_range)
        } 
        else if let Some(hit_record) = self.left.as_ref().unwrap().hit(ray, t_range.clone()) {
            if let Some(hit_record) = self.right.as_ref().unwrap().hit(ray, t_range.start..hit_record.t) {
                Some(hit_record)
            } else {
                Some(hit_record)
            }
        } else if let Some(hit_record) = self.right.as_ref().unwrap().hit(ray, t_range) {
            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

unsafe impl Send for BVH {}
unsafe impl Sync for BVH {}