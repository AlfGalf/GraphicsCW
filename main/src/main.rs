extern crate graphics_lib;

use glam::{Affine3A, Vec3};
use graphics_lib::camera::Camera;
use graphics_lib::color::Color;
use graphics_lib::lights::directional_light::DirectionalLight;
use graphics_lib::lights::point_light::PointLight;
use graphics_lib::materials::compound_material::CompoundMaterial;
use graphics_lib::materials::material::Material;
use graphics_lib::objects::csg::{CSGType, CSG};
use graphics_lib::objects::cube::Cube;
use graphics_lib::objects::object::Object;
use graphics_lib::objects::plane::Plane;
use graphics_lib::objects::poly_mesh::PolyMesh;
use graphics_lib::objects::sphere::Sphere;
use graphics_lib::scene::Scene;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let materials: Vec<Box<dyn Material + Sync>> = vec![
        Box::new(CompoundMaterial::new_transparent_material_opacity(
            // 0 -> transparent grey
            1.04,
            Color::new(0.7, 0.7, 0.7),
            0.17,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 1 -> reflective slight green
            Color::new(0.7, 0.8, 0.7),
            0.3,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 2 -> Red shiny
            Color::new(1., 0.6, 0.6) * 0.8,
            0.7,
        )),
        Box::new(
            CompoundMaterial::new_transparent_material(1.2), // 3 -> Transparent
        ),
        Box::new(CompoundMaterial::new_matte_material(
            // 4 -> Matte blue
            Color::new(0.5, 0.5, 1.0) * 0.7,
            0.2,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 5 -> dark slightly reflective
            Color::new(0.2, 0.2, 0.2),
            0.1,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 6 -> light grey reflective
            Color::new(0.5, 0.5, 0.5),
            0.4,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 7 -> blue matte
            Color::new(0.3, 0.3, 0.8),
            0.4,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 8 -> blue slightly reflective
            Color::new(0.8, 0.3, 0.3),
            0.3,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 9 -> grey matte
            Color::new(0.5, 0.5, 0.5),
            0.4,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 10 -> green matte
            Color::new(0.5, 0.7, 0.5),
            0.4,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 11 -> purple matte
            Color::new(0.7, 0.5, 0.7),
            0.3,
        )),
    ];

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut teapot = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.ply").unwrap()),
        2,
        true,
    )
    .unwrap();

    teapot.apply_transform(&Affine3A::from_cols_array(&[
        1.0, 0.0, 0.0, // 1
        0.0, 0.0, 1.0, // 2
        0.0, 1.0, 0.0, // 3
        -0.0, -2.0, 1.0, //
    ]));

    let mut castle = PolyMesh::from_file(
        BufReader::new(File::open("../castle.kcply").unwrap()),
        1,
        true,
    )
    .unwrap();

    castle.apply_transform(&Affine3A::from_cols_array(&[
        0.03, 0.0, 0.0, //1
        0.0, 0.0, 0.03, //2
        0.0, 0.03, 0.0, // 3
        1.0, -2.0, -2.0, //
    ]));

    let sphere = Sphere::new(
        Vec3::new(-0.2, 1.0, 0.),
        1.,
        6,
        // FalseColorMaterial::new(),
    );

    let sphere2 = Sphere::new(
        Vec3::new(0.2, 1.0, 0.),
        1.,
        6, // FalseColorMaterial::new(),
    );

    let sphere3 = Sphere::new(
        Vec3::new(1., 0.6, -0.8),
        0.6,
        100,
        // FalseColorMaterial::new(),
    );

    let plane_cut = Plane::new(Vec3::new(0., 1.2, 0.), Vec3::new(0.1, -0.1, 0.6), 100);

    let sphere4 = Sphere::new(
        Vec3::new(0.4, -0.5, -1.),
        2.0,
        1,
        // FalseColorMaterial::new(),
    );

    let plane_bottom = Plane::new(
        Vec3::new(0., -2., 0.),
        Vec3::new(0., 1., 0.),
        5,
        // FalseColorMaterial::new(),
    );

    let plane_back = Plane::new(
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 0., -1.),
        // FalseColorMaterial::new(),
        11,
    );

    let plane_left = Plane::new(
        Vec3::new(-4., 0., 0.),
        Vec3::new(1., 0., 0.),
        // FalseColorMaterial::new(),
        7,
    );

    let plane_right = Plane::new(
        Vec3::new(6., 0., 0.),
        Vec3::new(-1., 0., 0.),
        // FalseColorMaterial::new(),
        8,
    );

    let plane_top = Plane::new(
        Vec3::new(0., 4., 0.),
        Vec3::new(0., -1., 0.),
        // FalseColorMaterial::new(),
        9,
    );

    let plane_front = Plane::new(
        Vec3::new(0., 0., -25.),
        Vec3::new(0., 0., 1.),
        // FalseColorMaterial::new(),
        9,
    );

    // let union = CSG::new(CSGType::Union, Box::new(sphere), Box::new(sphere2), 0);
    let intersection = CSG::new(
        CSGType::Subtract,
        // Box::new(sphere),
        Box::new(CSG::new(
            CSGType::Union,
            Box::new(sphere),
            Box::new(sphere2),
        )),
        Box::new(sphere3),
    );

    let csg_cut = CSG::new(
        CSGType::Subtract,
        Box::new(intersection),
        Box::new(plane_cut),
    );

    let castle_hole = CSG::new(
        CSGType::Subtract,
        // Box::new(sphere),
        Box::new(castle),
        Box::new(sphere4),
    );

    let mut cube = Cube::new(3);
    cube.apply_transform(
        &(Affine3A::from_translation(Vec3::new(3., 0., -1.))
            * Affine3A::from_scale(Vec3::new(1.4, 2., 1.2))
            * Affine3A::from_rotation_y(0.2)
            * Affine3A::from_rotation_x(0.1)),
    );

    let light = PointLight::new(Vec3::new(-2.0, 4.0, -7.0), Color::new(0.9, 0.8, 0.85));
    let light2 = PointLight::new(Vec3::new(4.0, 4.0, -15.0), Color::new(0.8, 0.9, 0.85));
    let dir_light = DirectionalLight::new(Vec3::new(2.0, -4.0, 2.0), Color::new(0.9, 0.8, 0.85));

    let scene = Scene::new(
        vec![
            // Box::new(diffuse_sphere),
            // Box::new(teapot),
            // Box::new(sphere),
            // Box::new(sphere2),
            // Box::new(sphere3),
            // Box::new(intersection),
            Box::new(csg_cut),
            Box::new(plane_back),
            Box::new(plane_right),
            Box::new(plane_left),
            Box::new(plane_top),
            Box::new(plane_bottom),
            Box::new(plane_front),
            // Box::new(castle),
            // Box::new(castle_hole),
            // Box::new(cube),
        ],
        vec![
            Box::new(light),
            Box::new(light2),
            // Box::new(dir_light)
        ],
        materials,
        Camera::new(
            Vec3::new(0., 0., -20.),
            Vec3::new(0.05, 0.0, 1.0),
            Vec3::new(0., 1., 0.),
            2.2,
        ),
    );

    let fb = scene.render(1920, 1080);

    File::write_all(&mut file, &(fb.to_rgb_file())).unwrap();
}
