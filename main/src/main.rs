extern crate graphics_lib;

use glam::{Affine3A, Vec3};
use graphics_lib::cameras::dof_camera::DoFCamera;
use graphics_lib::cameras::normal_camera::NormalCamera;
use graphics_lib::color::Color;
use graphics_lib::lights::point_light::PointLight;
use graphics_lib::materials::compound_material::CompoundMaterial;
use graphics_lib::materials::false_color_material::FalseColorMaterial;
use graphics_lib::materials::material::Material;
use graphics_lib::objects::cube::Cube;
use graphics_lib::objects::object::Object;
use graphics_lib::objects::plane::Plane;
use graphics_lib::objects::poly_mesh::PolyMesh;
use graphics_lib::objects::sphere::Sphere;
use graphics_lib::scene::Scene;
use std::f32::consts::PI;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    let materials: Vec<Box<dyn Material + Sync + Send>> = vec![
        Box::new(CompoundMaterial::new_transparent_material_opacity(
            // 0 -> transparent grey
            1.2,
            Color::new(0.8, 1.0, 0.8),
            0.02,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 1 -> reflective slight green
            Color::new(1.0, 1.0, 1.0),
            0.9,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 2 -> Red shiny
            Color::new(1., 0.3, 0.3) * 0.8,
            0.5,
        )),
        Box::new(
            CompoundMaterial::new_transparent_material(1.1), // 3 -> Transparent
        ),
        Box::new(CompoundMaterial::new_matte_material(
            // 4 -> Matte blue
            Color::new(0.5, 0.5, 0.8),
            0.2,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 5 -> dark slightly reflective
            Color::new(0.2, 0.2, 0.2),
            0.3,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 6 -> light grey reflective
            Color::new(0.9, 0.9, 0.9),
            0.80,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 7 -> blue matte
            Color::new(0.3, 0.3, 0.8),
            0.4,
        )),
        Box::new(CompoundMaterial::new_matte_material(
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
        Box::new(FalseColorMaterial::new()), // 12 -> False color
    ];

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut castle = PolyMesh::from_file(
        BufReader::new(File::open("../castle.kcply").unwrap()),
        4,
        false,
    )
    .unwrap();

    castle.apply_transform(&Affine3A::from_cols_array(&[
        0.03, 0.0, 0.0, //1
        0.0, 0.0, 0.03, //2
        0.0, 0.03, 0.0, // 3
        -3.0, -2.0, -3.0, //
    ]));

    let plane_bottom = Plane::new(
        Vec3::new(0., -2., 0.),
        Vec3::new(0., 1., 0.),
        9,
        // FalseColorMaterial::new(),
    );

    let plane_back = Plane::new(
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 0., -1.),
        // FalseColorMaterial::new(),
        9,
    );

    let plane_left = Plane::new(
        Vec3::new(-4., 0., 0.),
        Vec3::new(1., 0., 0.),
        // FalseColorMaterial::new(),
        7,
    );

    let plane_right = Plane::new(
        Vec3::new(4., 0., 0.),
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

    let mut teapot_shiny = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.ply").unwrap()),
        1,
        true,
    )
    .unwrap();

    teapot_shiny.apply_transform(&Affine3A::from_cols_array(&[
        0.7, 0.0, 0.0, // 1
        0.0, 0.0, 0.7, // 2
        0.0, 0.7, 0.0, // 3
        -1.0, -1.0, -3.0, //
    ]));
    teapot_shiny.apply_transform(&Affine3A::from_rotation_x(0.));

    let mut teapot_trans_green = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.ply").unwrap()),
        0,
        true,
    )
    .unwrap();

    teapot_trans_green.apply_transform(&Affine3A::from_cols_array(&[
        0.7, 0.0, 0.0, // 1
        0.0, 0.0, 0.7, // 2
        0.0, 0.7, 0.0, // 3
        2.0, -1.4, -0.0, //
    ]));

    let light = PointLight::new(Vec3::new(0.1, 3.0, -10.0), Color::new(0.9, 0.4, 0.4));
    let light2 = PointLight::new(Vec3::new(-4.0, 3.0, -7.0), Color::new(0.4, 0.9, 0.4));
    let main_light = PointLight::new(Vec3::new(1.0, 3.5, -5.0), Color::new(0.9, 0.9, 0.9));

    let sphere = Sphere::new(Vec3::new(0., 1., 0.), 1.2, 1);

    let scene = Scene::new(
        vec![
            Box::new(plane_back),
            Box::new(plane_right),
            Box::new(plane_left),
            Box::new(plane_top),
            Box::new(plane_bottom),
            Box::new(plane_front),
            Box::new(sphere),
            // Box::new(teapot_shiny),
            // Box::new(teapot_trans_green),
        ],
        vec![Box::new(main_light)],
        materials,
        Box::new(NormalCamera::new(
            Vec3::new(1., 1., -20.),
            Vec3::new(-0.05, 0., 1.0),
            Vec3::new(0., 1., 0.),
            1.7,
        )),
        // Box::new(DoFCamera::new(
        //     Vec3::new(3., 0., -20.),
        //     Vec3::new(-0.10, 0., 1.0),
        //     Vec3::new(0., 1., 0.),
        //     1.7,
        //     10,
        //     20.,
        //     0.5,
        // )),
    );

    let fb = scene.render(400, 400);

    File::write_all(&mut file, &(fb.to_rgb_file(0.5))).unwrap();
}
