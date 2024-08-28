use std::sync::Arc;

use raytracer::{camera::Camera, hittable::{sphere::Sphere, world::World}, material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal}, math::vec3::Vec3, pipeline::{descriptor::{ImageDescriptor, InstanceDescriptor, SamplePointGeneratorDescriptor, SamplerDescriptor}, instance::Instance}, utils::random::{random, random_range}, Float};

#[tokio::main(flavor = "multi_thread", worker_threads=10)]
async fn main() {
    let world = Arc::new(build_world());
    let camera = Camera::new(
        10.0,
        0.6,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0
    );
    let instance = Instance::new(InstanceDescriptor {
        point_generator_descriptor: SamplePointGeneratorDescriptor {
            num_threads: 1,
            buffer_size: 4096,
            image: ImageDescriptor {
                width: 1920,
                height: 1080,
                samples_per_pixel: 10,
            },
            camera,
        },
        sampler_descriptor: SamplerDescriptor {
            num_threads: 10,
            in_buffer_size: 4096,
            feedback_buffer_size: 20480,
            out_buffer_size: 20480,
            max_bounces: 5,
        },
        progressbar: true
    });

    let image = instance.begin(world).await.expect("failed to generate image");
    image.save("output/output.png");
}

fn build_world() -> World {
    let world = World::new();
    let world = build_materials(world);
    build_objects(world)
}

fn build_objects(mut world: World) -> World {
    world.add_hittable(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        world.get_material("ground").unwrap(),
    )));
    world.add_hittable(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        world.get_material("dielectric").unwrap(),
    )));
    world.add_hittable(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        world.get_material("lambertian").unwrap(),
    )));
    world.add_hittable(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        world.get_material("metal").unwrap(),
    )));

    for x in -11..11 {
        for z in -11..11 {
            let material = {
                let rand = random::<Float>();
                let mat_type = if rand < 0.5 {
                    "lambertian"
                } else if rand < 0.9 {
                    "metal"
                } else {
                    "dielectric"
                };
                world.get_material(format!("{}{}", mat_type, random_range(0..10)).as_str()).unwrap()
            };
            let center = Vec3::new(x as Float, 0.2, z as Float) 
                             + Vec3::new(random_range(0.0f32..0.9f32), 0.0, random_range(0.0f32..0.9f32));
            world.add_hittable(Box::new(Sphere::new(center, 0.2, material)));
        }
    }

    world
}

fn build_materials(mut world: World) -> World {
    world.add_material("ground", Box::new(Lambertian::new(
        Vec3::new(0.5, 0.5, 0.5),
    )));
    world.add_material("dielectric", Box::new(Dielectric::new(
        Vec3::new_diagonal(1.0), 1.5
    )));
    world.add_material("metal", Box::new(Metal::new(
        Vec3::new(0.7, 0.6, 0.5),
        0.1,
    )));
    world.add_material("lambertian", Box::new(Lambertian::new(
        Vec3::new(0.4, 0.2, 0.1),
    )));

    for i in 0..10 {
        world.add_material(format!("lambertian{}", i).as_str(), Box::new(Lambertian::new(
            Vec3::new_random() * Vec3::new_random(),
        )));
        world.add_material(format!("metal{}", i).as_str(), Box::new(Metal::new(
            Vec3::new_random_range(0.5..1.0),
            random_range(0.0..0.5),
        )));
        world.add_material(format!("dielectric{}", i).as_str(), Box::new(Dielectric::new(
            Vec3::new_random_range(0.5..1.0),
            random_range(0.5..1.5),
        )));
    }

    world
}
