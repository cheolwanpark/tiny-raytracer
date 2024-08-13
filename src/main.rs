use raytracer::{
    camera::Camera,
    hittable::{list::HittableList, sphere::Sphere},
    math::vec3::Vec3,
    renderer::{
        bruteforce::BruteForceRenderer, colorsampler::GeneralSampler, ImageOptions, Renderer,
    },
    Float,
};

fn main() {
    let width = 400_usize;
    let aspect_ratio = 16.0 / 9.0;
    let image_options = ImageOptions::new(width, (width as Float / aspect_ratio) as usize);
    let camera = Camera::new(
        1.0,
        Vec3::zero(),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
    );

    let _renderer = BruteForceRenderer::new(Box::new(GeneralSampler::new()), 50, 10);

    let mut world = Box::new(HittableList::new());
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let renderer = BruteForceRenderer::new(Box::new(GeneralSampler::new()), 50, 10);
    let image = renderer.render(camera, world, image_options);
    image.save("output.png")
}
