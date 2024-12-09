use std::{mem::transmute, sync::Arc};

use metal::{AccelerationStructureBoundingBoxGeometryDescriptor, Buffer, Device, MTLResourceOptions, NSRange, NSUInteger};

use crate::{hittable::{aabb::AABB, sphere::Sphere, Hittable}, material::Material, math::vec3::Vec3, Float};

use super::MetalGeometry;

#[repr(C)]
struct MetalSphere {
    center: Vec3,
    radius: Float,
    idx: u32,
}

pub struct MetalSphereGeometry {
    device: Device,
    sphere_buffer: Option<Buffer>,
    bbox_buffer: Option<Buffer>,
    spheres: Vec<MetalSphere>,
    bboxes: Vec<AABB>,
    materials: Vec<Arc<Box<dyn Material>>>,
}

impl MetalSphereGeometry {
    pub fn new(device: Device, spheres: Vec<Sphere>) -> Self {
        let metal_spheres: Vec<MetalSphere> = spheres.iter().enumerate().map(|(i, s)| MetalSphere {
            center: s.center,
            radius: s.radius,
            idx: i as u32,
        }).collect();
        let bboxes: Vec<AABB> = spheres.iter().map(|s| s.bounding_box()).collect();
        let materials: Vec<Arc<Box<dyn Material>>> = spheres.iter().map(|s| s.material.clone()).collect();

        Self {
            device,
            sphere_buffer: None,
            bbox_buffer: None,
            spheres: metal_spheres,
            bboxes,
            materials
        }
    }
}

impl MetalGeometry for MetalSphereGeometry {
    fn upload_to_buffers(&mut self) {
        if self.sphere_buffer.is_none() {
            self.sphere_buffer = Some(unsafe {
                self.device.new_buffer_with_data(
                    transmute(self.spheres.as_ptr()),
                    (self.spheres.len() * size_of::<MetalSphere>()) as NSUInteger,
                    self.get_resource_option())
            });
            self.sphere_buffer.as_ref().unwrap().set_label("sphere buffer");
            self.sphere_buffer.as_ref().unwrap().did_modify_range(
                NSRange::new(0, self.sphere_buffer.as_ref().unwrap().length())
            );
        }
        if self.bbox_buffer.is_none() {
            self.bbox_buffer = Some(unsafe {
                self.device.new_buffer_with_data(
                    transmute(self.bboxes.as_ptr()),
                    (self.bboxes.len() * size_of::<AABB>()) as NSUInteger,
                    self.get_resource_option())
            });
            self.bbox_buffer.as_ref().unwrap().set_label("bbox buffer");
            self.bbox_buffer.as_ref().unwrap().did_modify_range(
                NSRange::new(0, self.bbox_buffer.as_ref().unwrap().length())
            );
        }
    }

    fn get_geometry_descriptor(&self) -> metal::AccelerationStructureGeometryDescriptor {
        let desc = AccelerationStructureBoundingBoxGeometryDescriptor::descriptor();
        desc.set_bounding_box_buffer(Some(self.bbox_buffer.as_ref().unwrap()));
        desc.set_bounding_box_count(self.spheres.len() as NSUInteger);
        desc.set_primitive_data_buffer(Some(self.sphere_buffer.as_ref().unwrap()));
        desc.set_primitive_data_stride(size_of::<MetalSphere>() as NSUInteger);
        desc.set_primitive_data_element_size(size_of::<MetalSphere>() as NSUInteger);
        From::from(desc)
    }

    fn get_intersection_function_name(&self) -> Option<&str> {
        Some("sphereIntersectionFunction")
    }
}