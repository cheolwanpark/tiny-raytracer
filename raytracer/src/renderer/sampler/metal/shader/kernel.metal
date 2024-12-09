#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;
using namespace raytracing;

#include "geometry.metal"

struct Ray {
    packed_float3 origin;
    packed_float3 direction;
};

struct HitRecord {
    packed_float3 p;
    packed_float3 normal;
    float t;
    uint mask;
    uint idx;
    bool hit;
};

kernel void raytracingKernel(
     uint gid                           [[thread_position_in_grid]],
     device Ray *rays                   [[buffer(0)]],
     volatile device HitRecord *recs    [[buffer(1)]],
     constant MTLAccelerationStructureInstanceDescriptor *instances                     [[buffer(2)]],
     instance_acceleration_structure accelerationStructure                              [[buffer(3)]],
     intersection_function_table<triangle_data, instancing> intersectionFunctionTable   [[buffer(4)]]
) {
    Ray input_ray = rays[gid];
    ray ray;
    ray.origin = input_ray.origin;
    ray.direction = input_ray.direction;
    ray.max_distance = INFINITY;

    intersector<triangle_data, instancing> i;
    typename intersector<triangle_data, instancing>::result_type intersection;

    intersection = i.intersect(ray, accelerationStructure, GEOMETRY_MASK_ALL, intersectionFunctionTable);

    if(intersection.type == intersection_type::none) {
        recs[gid].hit = false;
        return;
    }


    uint instanceIndex = intersection.instance_id;
    uint mask = instances[instanceIndex].mask;

    volatile device HitRecord *rec = &recs[gid];
    rec->hit = true;
    rec->t = intersection.distance;
    rec->p = ray.origin + ray.direction*rec->t;
    rec->mask = mask;

    if(mask & GEOMETRY_MASK_SPHERE) {
        Sphere sphere = *(const device Sphere*)intersection.primitive_data;
        
        rec->idx = sphere.idx;
        rec->normal = rec->p - sphere.center;
        if(dot(rec->normal, ray.direction) >= 0.0f) {
            rec->normal = -rec->normal;
        }
    } else if(mask & GEOMETRY_MASK_QUAD) {
        Quad quad = *(const device Quad*)intersection.primitive_data;

        rec->idx = quad.idx;
        rec->normal = quad.n;
        if(dot(rec->normal, ray.direction) >= 0.0f) {
            rec->normal = -rec->normal;
        }
    }
}