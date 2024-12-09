#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;
using namespace raytracing;

#define GEOMETRY_MASK_SPHERE    1
#define GEOMETRY_MASK_QUAD      2
#define GEOMETRY_MASK_ALL       (GEOMETRY_MASK_SPHERE | GEOMETRY_MASK_QUAD)

struct Sphere {
    packed_float3 center;
    float radius;
    uint idx;
};

struct Quad {
    packed_float3 corner;
    packed_float3 u;
    packed_float3 v;
    packed_float3 n;
    packed_float3 w;
    float d;
    uint idx;
};

struct BoundingBoxIntersection {
    bool hit    [[accept_intersection]];
    float t     [[distance]];
};


[[intersection(bounding_box, triangle_data, instancing)]]
BoundingBoxIntersection sphereIntersectionFunction(
    float3 origin                       [[origin]],
    float3 direction                    [[direction]],
    float minDistance                   [[min_distance]],
    float maxDistance                   [[max_distance]],
    unsigned int primitiveIndex         [[primitive_id]],
    unsigned int geometryIndex          [[geometry_intersection_function_table_offset]],
    const device void* perPrimitiveData [[primitive_data]]
) {
    Sphere sphere = *(const device Sphere*)perPrimitiveData;
    BoundingBoxIntersection ret;

    float3 oc = origin - sphere.center;
    float a = dot(direction, direction);
    float half_b = dot(oc, direction);
    float c = dot(oc, oc) - sphere.radius*sphere.radius;

    float disc = half_b*half_b - a*c;

    if(disc <= 0.0f) {
        ret.hit = false;
    } else {
        float sqrtd = sqrt(disc);
        ret.t = (-half_b - sqrtd) / a;
        ret.hit = true;
        if(ret.t < minDistance || maxDistance < ret.t) {
            ret.t = (-half_b + sqrtd) / a;
            if(ret.t < minDistance || maxDistance < ret.t) {
                ret.hit = false;
            }
        }
    }

    return ret;
}

[[intersection(bounding_box, triangle_data, instancing)]]
BoundingBoxIntersection quadIntersectionFunction(
    float3 origin                       [[origin]],
    float3 direction                    [[direction]],
    float minDistance                   [[min_distance]],
    float maxDistance                   [[max_distance]],
    unsigned int primitiveIndex         [[primitive_id]],
    unsigned int geometryIndex          [[geometry_intersection_function_table_offset]],
    const device void* perPrimitiveData [[primitive_data]]
) {
    Quad quad = *(const device Quad*)perPrimitiveData;
    BoundingBoxIntersection ret;
    ret.hit = false;

    float dir_norm = dot(direction, quad.n);
    float t = (quad.d - dot(origin, quad.n)) / dir_norm;

    if(minDistance <= t && t <= maxDistance) {
        float3 p = origin + direction*t - quad.corner;
        float planar_x = dot(cross(p, quad.v), quad.w);
        float planar_y = dot(cross(quad.u, p), quad.w);
        if(0.0f <= planar_x && planar_x <= 1.0 &&
           0.0f <= planar_y && planar_y <= 1.0) {
            ret.hit = true;
            ret.t = t;
        }
    }

    return ret;
}