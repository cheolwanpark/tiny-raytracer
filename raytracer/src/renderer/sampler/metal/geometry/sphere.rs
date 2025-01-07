use std::{mem::transmute, sync::Arc};

use metal::{AccelerationStructureBoundingBoxGeometryDescriptor, Buffer, Device, MTLResourceOptions, NSRange, NSUInteger};

use crate::{hittable::{aabb::AABB, sphere::Sphere, Hittable}, material::Material, math::vec3::Vec3, renderer::sampler::metal::global_resource_option, Float};

use super::{masks::MetalGeometryMasks, MetalGeometry};

#[repr(C)]
struct MetalSphere {
    center: Vec3,
    radius: Float,
    idx: u32,
}

pub struct MetalSphereGeometry {
    device: Device,
    sphere_buffer: Buffer,
    bbox_buffer: Buffer,
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

        let sphere_buffer = device.new_buffer_with_data(
            unsafe{ transmute(metal_spheres.as_ptr()) },
            (metal_spheres.len() * size_of::<MetalSphere>()) as NSUInteger,
            global_resource_option());
        sphere_buffer.set_label("sphere buffer");
        sphere_buffer.did_modify_range(
            NSRange::new(0, sphere_buffer.length())
        );
        
        let bbox_buffer = device.new_buffer_with_data(
            unsafe { transmute(bboxes.as_ptr()) },
            (bboxes.len() * size_of::<AABB>()) as NSUInteger,
            global_resource_option());
        bbox_buffer.set_label("bbox buffer");
        bbox_buffer.did_modify_range(
            NSRange::new(0, bbox_buffer.length())
        );

        Self {
            device,
            sphere_buffer,
            bbox_buffer,
            spheres: metal_spheres,
            bboxes,
            materials
        }
    }
}

impl MetalGeometry for MetalSphereGeometry {
    fn get_geometry_descriptor(&self) -> metal::AccelerationStructureGeometryDescriptor {
        let desc = AccelerationStructureBoundingBoxGeometryDescriptor::descriptor();
        desc.set_bounding_box_buffer(Some(&self.bbox_buffer));
        desc.set_bounding_box_count(self.spheres.len() as NSUInteger);
        desc.set_primitive_data_buffer(Some(&self.sphere_buffer));
        desc.set_primitive_data_stride(size_of::<MetalSphere>() as NSUInteger);
        desc.set_primitive_data_element_size(size_of::<MetalSphere>() as NSUInteger);
        From::from(desc)
    }

    fn get_intersection_function_name(&self) -> Option<&str> {
        Some("sphereIntersectionFunction")
    }

    fn get_mask(&self) -> u32 {
        MetalGeometryMasks::Sphere.into()
    }
}