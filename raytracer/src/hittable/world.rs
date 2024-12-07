use std::{collections::HashMap, ops::Range, sync::Arc};

use crate::{accel::{aabb::AABB, bvh::BVH}, hittable::{list::HittableList, HitRecord, Hittable}, material::Material, ray::Ray, Float};

pub struct World {
    hittable_root: HittableList,
    materials: HashMap<String, Arc<Box<dyn Material>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            hittable_root: HittableList::new(),
            materials: HashMap::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.hittable_root.push(hittable)
    }

    pub fn add_material(&mut self, name: &str, material: Box<dyn Material>) {
        let name = name.to_string();
        if self.materials.contains_key(&name) {
            panic!("{} key is already in the material table", name.as_str());
        }
        self.materials.insert(name, Arc::new(material));
    }

    pub fn get_material(&self, name: &str) -> Option<Arc<Box<dyn Material>>> {
        if let Some(material) = self.materials.get(&name.to_string()) {
            Some(material.clone())
        } else {
            None
        }
    }

    pub fn get_bvh(&self) -> BVH {
        BVH::new(&self.hittable_root)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_range: Range<Float>) -> Option<HitRecord> {
        self.hittable_root.hit(ray, t_range)
    }

    fn bounding_box(&self) -> AABB {
        self.hittable_root.bounding_box()
    }
}

unsafe impl Send for World {}
unsafe impl Sync for World {}