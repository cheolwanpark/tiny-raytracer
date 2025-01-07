use std::{mem::transmute, sync::Arc};

use metal::{AccelerationStructureBoundingBoxGeometryDescriptor, Buffer, Device, NSRange, NSUInteger};

use crate::{hittable::{aabb::AABB, quad::Quad, Hittable}, material::Material, math::vec3::Vec3, renderer::sampler::metal::global_resource_option, Float};

use super::{masks::MetalGeometryMasks, MetalGeometry};

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
    quad_buffer: Buffer,
    bbox_buffer: Buffer,
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

        let quad_buffer = device.new_buffer_with_data(
            unsafe{ transmute(metal_quads.as_ptr()) },
            (metal_quads.len() * size_of::<MetalQuad>()) as NSUInteger,
            global_resource_option());
        quad_buffer.set_label("quad buffer");
        quad_buffer.did_modify_range(
            NSRange::new(0, quad_buffer.length())
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
            quad_buffer,
            bbox_buffer,
            quads: metal_quads,
            bboxes,
            materials,
        }
    }
}

impl MetalGeometry for MetalQuadGeometry {
    fn get_geometry_descriptor(&self) -> metal::AccelerationStructureGeometryDescriptor {
        let desc = AccelerationStructureBoundingBoxGeometryDescriptor::descriptor();
        desc.set_bounding_box_buffer(Some(&self.bbox_buffer));
        desc.set_bounding_box_count(self.quads.len() as NSUInteger);
        desc.set_primitive_data_buffer(Some(&self.quad_buffer));
        desc.set_primitive_data_stride(size_of::<MetalQuad>() as NSUInteger);
        desc.set_primitive_data_element_size(size_of::<MetalQuad>() as NSUInteger);
        From::from(desc)
    }

    fn get_intersection_function_name(&self) -> Option<&str> {
        Some("quadIntersectionFunction")
    }

    fn get_mask(&self) -> u32 {
        MetalGeometryMasks::Quad.into()
    }
}