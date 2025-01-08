use std::{collections::HashMap, ops::Range, sync::Arc};

use as_any::Downcast;
use metal::Device;

use crate::{hittable::{HitRecord, Hittable}, material::Material, ray::Ray, renderer::sampler::metal::geometry::{quad::MetalQuadGeometry, sphere::MetalSphereGeometry, MetalGeometry}, Float};

use super::{aabb::AABB, bvh::BVH, quad::Quad, sphere::Sphere};

pub struct World {
    geometries: Vec<Arc<Box<dyn Hittable>>>,
    materials: HashMap<String, Arc<Box<dyn Material>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            geometries: Vec::new(),
            materials: HashMap::new(),
        }
    }

    pub fn add_geometry(&mut self, geometry: Box<dyn Hittable>) {
        self.geometries.push(Arc::new(geometry));
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
        BVH::new(&self.geometries)
    }

    pub fn get_geometries<T: Downcast + Clone>(&self) -> Vec<T> {
        let mut geometries = Vec::new();
        for object in &self.geometries {
            if let Some(geometry) = (***object).downcast_ref::<T>() {
                geometries.push(geometry.clone());
            }
        }
        geometries
    }

    pub fn get_materials<T: Downcast + Clone>(&self) -> Vec<T> {
        let mut materials = Vec::new();
        for (_, object) in &self.materials {
            if let Some(material) = (***object).downcast_ref::<T>() {
                materials.push(material.clone());
            }
        }
        materials
    }

    pub fn get_metal_geometries(&self, device: Device) -> Vec<Box<dyn MetalGeometry>> {
        let mut geometries: Vec<Box<dyn MetalGeometry>> = Vec::new();
        let quads = self.get_geometries::<Quad>();
        let spheres = self.get_geometries::<Sphere>();
        if !quads.is_empty() {
            geometries.push(Box::new(MetalQuadGeometry::new(device.clone(), quads)));
        }
        if !spheres.is_empty() {
            geometries.push(Box::new(MetalSphereGeometry::new(device.clone(), spheres)));
        }
        geometries
    }
}

unsafe impl Send for World {}
unsafe impl Sync for World {}