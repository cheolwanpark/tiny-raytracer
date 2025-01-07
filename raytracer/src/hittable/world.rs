use std::{collections::HashMap, ops::Range, sync::Arc};

use metal::Device;

use crate::{hittable::{list::HittableList, HitRecord, Hittable}, material::Material, ray::Ray, renderer::sampler::metal::geometry::{quad::MetalQuadGeometry, sphere::MetalSphereGeometry, MetalGeometry}, Float};

use super::{aabb::AABB, bvh::BVH, quad::Quad, sphere::Sphere};

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

    pub fn get_metal_geometries(&self, device: Device) -> Vec<Box<dyn MetalGeometry>> {
        let mut geometries: Vec<Box<dyn MetalGeometry>> = Vec::new();
        let quads = self.hittable_root.get_geometries::<Quad>();
        let spheres = self.hittable_root.get_geometries::<Sphere>();
        if !quads.is_empty() {
            geometries.push(Box::new(MetalQuadGeometry::new(device.clone(), quads)));
        }
        if !spheres.is_empty() {
            geometries.push(Box::new(MetalSphereGeometry::new(device.clone(), spheres)));
        }
        geometries
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