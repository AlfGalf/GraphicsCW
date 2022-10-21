extern crate graphics_lib;

use glam::{Affine3A, Vec3};
use graphics_lib::camera::Camera;
use graphics_lib::color::Color;
use graphics_lib::lights::directional_light::DirectionalLight;
use graphics_lib::lights::point_light::PointLight;
use graphics_lib::materials::compound_material::CompoundMaterial;
use graphics_lib::materials::diffuse_material::DiffuseMaterial;
use graphics_lib::objects::object::Object;
use graphics_lib::objects::plane::Plane;
use graphics_lib::objects::poly_mesh::PolyMesh;
use graphics_lib::objects::sphere::Sphere;
use graphics_lib::scene::Scene;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut teapot = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.ply").unwrap()),
        graphics_lib::materials::compound_material::CompoundMaterial::new_matte_material(
            Color::new(0.1, 0.6, 0.1),
            0.4,
        ),
        // FalseColorMaterial::new(),
        false,
    )
    .unwrap();

    teapot.apply_transform(&Affine3A::from_cols_array(&[
        1.0, 0.0, 0.0, //1
        0.0, 0.0, 1.0, //2
        0.0, 1.0, 0.0, // 3
        -1.0, -2.0, -1.0, //
    ]));

    let sphere = Sphere::new(
        Vec3::new(1., 1., -0.5),
        1.,
        CompoundMaterial::new_reflective_material(Color::new(1., 0.5, 0.5) * 0.7, 0.6),
        // FalseColorMaterial::new(),
    );

    let sphere3 = Sphere::new(
        Vec3::new(3.5, 2.5, -1.),
        1.,
        CompoundMaterial::new_reflective_material(Color::new(0.5, 1.0, 0.5) * 0.7, 0.6),
        // FalseColorMaterial::new(),
    );

    let sphere2 = Sphere::new(
        Vec3::new(3.5, 0.0, 0.),
        1.,
        CompoundMaterial::new_matte_material(Color::new(0.5, 0.5, 1.0) * 0.7, 0.2),
        // FalseColorMaterial::new(),
    );

    let plane_bottom = Plane::new(
        Vec3::new(0., -2., 0.),
        Vec3::new(0., 1., 0.),
        CompoundMaterial::new_reflective_material(Color::new(0.2, 0.2, 0.2), 0.1),
        // FalseColorMaterial::new(),
    );

    let plane_back = Plane::new(
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 0., -1.),
        // FalseColorMaterial::new(),
        CompoundMaterial::new_reflective_material(Color::new(0.5, 0.5, 0.5), 0.4),
    );

    let plane_left = Plane::new(
        Vec3::new(-4., 0., 0.),
        Vec3::new(1., 0., 0.),
        // FalseColorMaterial::new(),
        CompoundMaterial::new_matte_material(Color::new(0.3, 0.3, 0.8), 0.4),
    );

    let plane_right = Plane::new(
        Vec3::new(6., 0., 0.),
        Vec3::new(-1., 0., 0.),
        // FalseColorMaterial::new(),
        CompoundMaterial::new_reflective_material(Color::new(0.8, 0.3, 0.3), 0.3),
    );

    let plane_top = Plane::new(
        Vec3::new(0., 4., 0.),
        Vec3::new(0., -1., 0.),
        // FalseColorMaterial::new(),
        CompoundMaterial::new_matte_material(Color::new(0.5, 0.5, 0.5), 0.4),
    );

    let plane_front = Plane::new(
        Vec3::new(0., 0., -25.),
        Vec3::new(0., 0., 1.),
        // FalseColorMaterial::new(),
        CompoundMaterial::new_matte_material(Color::new(0.5, 0.5, 0.5), 0.4),
    );

    let diffuse_sphere = Sphere::new(
        Vec3::new(1., 1., -0.5),
        1.,
        DiffuseMaterial::new(),
        // FalseColorMaterial::new(),
    );

    let light = PointLight::new(Vec3::new(-2.0, 4.0, -7.0), Color::new(0.9, 0.8, 0.85));
    let light2 = PointLight::new(Vec3::new(4.0, 4.0, -15.0), Color::new(0.8, 0.9, 0.85));
    let dir_light = DirectionalLight::new(Vec3::new(2.0, -4.0, 2.0), Color::new(0.9, 0.8, 0.85));

    let scene = Scene::new(
        vec![
            // Box::new(diffuse_sphere),
            Box::new(teapot),
            Box::new(sphere),
            Box::new(sphere2),
            Box::new(sphere3),
            Box::new(plane_back),
            Box::new(plane_right),
            Box::new(plane_left),
            Box::new(plane_top),
            Box::new(plane_bottom),
            Box::new(plane_front),
        ],
        vec![
            Box::new(light),
            Box::new(light2),
            // Box::new(dir_light)
            //
        ],
        Camera::new(
            Vec3::new(0., 0., -20.),
            Vec3::new(0.05, 0.0, 1.0),
            Vec3::new(0., 1., 0.),
            2.0,
        ),
    );

    let fb = scene.render(960, 540);

    File::write_all(&mut file, &(fb.to_rgb_file())).unwrap();
}
