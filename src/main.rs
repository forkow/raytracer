extern crate nalgebra as na;

mod camera;
mod color;
mod math;
mod sampler;
mod scene;
mod world;

use na::Vector3;
use scene::{RenderInfo, Scene};

use crate::{
    camera::Camera,
    color::Color,
    sampler::{Image2DSampler, Sampler2D},
    world::{
        material::{metal::Metallic, Material},
        object::{sphere::Sphere, Object},
        World,
    },
};


fn make_world() -> World {
    // let earth_image =
    //     Image::load_from_png(File::open("res/earthmap.png").unwrap()).unwrap();
    // let image1 =
    //     Image::load_from_png(File::open("res/earthmap.png").unwrap()).unwrap();

    let mut world = World::new();

    let image1 = image::open("res/concrete.png").unwrap();

    world.add_object(Object::Sphere(Sphere::new(
        Vector3::new(0.0, 0.0, -2.5),
        1.0,
        Material::Metallic(Metallic::new(
            Sampler2D::Image(Image2DSampler::new(
                image1,
            )),
            1.0,
        )),
    )));
    world.add_object(Object::Sphere(Sphere::new(
        Vector3::new(1.414, -0.5, -2.0),
        0.5,
        Material::Metallic(Metallic::new(
            Sampler2D::Static(Color::gray(1.0)),
            0.0,
        )),
    )));

    world.add_object(Object::Sphere(Sphere::new(
        Vector3::new(-1.414, -0.5, -2.0),
        0.5,
        Material::Metallic(Metallic::new(
            Sampler2D::Static(Color::gray(1.0)),
            0.5,
        )),
    )));

    world.add_object(Object::Sphere(Sphere::new(
        Vector3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Metallic(Metallic::new(
            Sampler2D::Static(Color::rgb(0.5, 0.5, 0.8)),
            1.0,
        )),
    )));
    world
}

fn main() {
    let world = make_world();
    let camera = Camera::new(16.0 / 9.0);

    let scene = Scene::new(world, camera);

    let image = scene.render(RenderInfo {
        width: 1280,
        height: 720,
        max_depth: 256,
        gamma: 2.2,
        samples: 128,
    });

    image.save("out/render.png").unwrap();
    println!("Output saved to out/render.png");
}
