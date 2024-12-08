use std::sync::Arc;

use raytracer::{camera::Camera, hittable::{list::HittableList, quad::Quad, world::World}, material::{lambertian::Lambertian, light::Light, Material}, math::vec3::Vec3, renderer::Renderer};

#[tokio::main(flavor = "multi_thread", worker_threads=8)]
async fn main() {
    let world = Arc::new(build_world());
    let camera = Camera::new(
        140.0,
        0.6,
        Vec3::new(50.0, 50.0, -140.0),
        Vec3::new(50.0, 50.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        300, 300,
    );
    let instance = Renderer::new(1000, 8, 20, true, None);

    let image = instance.render(camera, world).await.expect("failed to generate image");
    image.save("output/output.png");
}

fn build_world() -> World {
    let world = World::new();
    let world = build_materials(world);
    build_objects(world)
}

fn build_objects(mut world: World) -> World {
    world.add_hittable(Box::new(Quad::new(
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        world.get_material("green").unwrap()
    )));
    world.add_hittable(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        world.get_material("red").unwrap()
    )));
    world.add_hittable(Box::new(Quad::new(
        Vec3::new(65.0, 100.0, 60.0),
        Vec3::new(-30.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -20.0),
        world.get_material("light").unwrap()
    )));
    world.add_hittable(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        world.get_material("white").unwrap()
    )));
    world.add_hittable(Box::new(Quad::new(
        Vec3::new(100.0, 100.0, 100.0),
        Vec3::new(-100.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -100.0),
        world.get_material("white").unwrap()
    )));
    world.add_hittable(Box::new(Quad::new(
        Vec3::new(0.0, 0.0, 100.0),
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        world.get_material("white").unwrap()
    )));
    world.add_hittable(Box::new(new_box(
        Vec3::new(25.0, 0.0, 50.0),
        Vec3::new(55.0, 60.0, 80.0),
        world.get_material("white").unwrap()
    )));
    world.add_hittable(Box::new(new_box(
        Vec3::new(45.0, 0.0, 10.0),
        Vec3::new(75.0, 30.0, 40.0),
        world.get_material("white").unwrap()
    )));

    world
}

fn build_materials(mut world: World) -> World {
    world.add_material("red", Box::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))));
    world.add_material("white", Box::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))));
    world.add_material("green", Box::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))));
    world.add_material("light", Box::new(Light::new(Vec3::new_diagonal(15.0))));

    world
}

fn new_box(a: Vec3, b: Vec3, mat: Arc<Box<dyn Material>>) -> HittableList {
    let mut instance = HittableList::new();

    let min = Vec3::new_min(a, b);
    let max = Vec3::new_max(a, b);

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    instance.push(Box::new(Quad::new(
        Vec3::new(min.x, min.y, max.z),
        dx, dy, mat.clone()
    )));
    instance.push(Box::new(Quad::new(
        Vec3::new(max.x, min.y, max.z),
        -dz, dy, mat.clone()
    )));
    instance.push(Box::new(Quad::new(
        Vec3::new(max.x, min.y, min.z),
        -dx, dy, mat.clone()
    )));
    instance.push(Box::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dz, dy, mat.clone()
    )));
    instance.push(Box::new(Quad::new(
        Vec3::new(min.x, max.y, max.z),
        dx, -dz, mat.clone()
    )));
    instance.push(Box::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dx, dz, mat
    )));

    instance
}