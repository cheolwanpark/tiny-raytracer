use std::{mem::transmute, sync::Arc};

use metal::{AccelerationStructureBoundingBoxGeometryDescriptor, Buffer, Device, NSRange, NSUInteger};

use crate::{hittable::{aabb::AABB, quad::Quad, Hittable}, material::Material, math::vec3::Vec3, Float};

use super::MetalGeometry;

#[repr(C)]
struct MetalQuad {
    corner: Vec3,
    u: Vec3,
    v: Vec3,
    n: Vec3,
    w: Vec3,
    d: Float,
    idx: u32,
}

pub struct MetalQuadGeometry {
    device: Device,
    quad_buffer: Option<Buffer>,
    bbox_buffer: Option<Buffer>,
    quads: Vec<MetalQuad>,
    bboxes: Vec<AABB>,
    materials: Vec<Arc<Box<dyn Material>>>,
}

impl MetalQuadGeometry {
    pub fn new(device: Device, quads: Vec<Quad>) -> Self {
        let metal_quads: Vec<MetalQuad> = quads.iter().enumerate().map(|(i, q)| MetalQuad {
            corner: q.corner,
            u: q.u,
            v: q.v,
            n: q.n,
            w: q.w,
            d: q.d,
            idx: i as u32,
        }).collect();
        let bboxes: Vec<AABB> = quads.iter().map(|q| q.bounding_box()).collect();
        let materials: Vec<Arc<Box<dyn Material>>> = quads.iter().map(|q| q.material.clone()).collect();

        Self {
            device,
            quad_buffer: None,
            bbox_buffer: None,
            quads: metal_quads,
            bboxes,
            materials,
        }
    }
}


impl MetalGeometry for MetalQuadGeometry {
    fn upload_to_buffers(&mut self) {
        if self.quad_buffer.is_none() {
            self.quad_buffer = Some(unsafe {
                self.device.new_buffer_with_data(
                    transmute(self.quads.as_ptr()),
                    (self.quads.len() * size_of::<MetalQuad>()) as NSUInteger,
                    self.get_resource_option())
            });
            self.quad_buffer.as_ref().unwrap().set_label("quad buffer");
            self.quad_buffer.as_ref().unwrap().did_modify_range(
                NSRange::new(0, self.quad_buffer.as_ref().unwrap().length())
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
        desc.set_bounding_box_count(self.quads.len() as NSUInteger);
        desc.set_primitive_data_buffer(Some(self.quad_buffer.as_ref().unwrap()));
        desc.set_primitive_data_stride(size_of::<MetalQuad>() as NSUInteger);
        desc.set_primitive_data_element_size(size_of::<MetalQuad>() as NSUInteger);
        From::from(desc)
    }

    fn get_intersection_function_name(&self) -> Option<&str> {
        Some("quadIntersectionFunction")
    }
}