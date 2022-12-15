extern crate graphics_lib;

use glam::{DAffine3, DVec3};
use graphics_lib::cameras::dof_camera::DoFCamera;
use graphics_lib::cameras::normal_camera::NormalCamera;
use graphics_lib::color::Color;
use graphics_lib::lights::point_light::PointLight;
use graphics_lib::materials::compound_material::CompoundMaterial;
use graphics_lib::materials::false_color_material::FalseColorMaterial;
use graphics_lib::materials::material::Material;
use graphics_lib::objects::csg::CSGType::{Intersection, Subtract, Union};
use graphics_lib::objects::csg::CSG;
use graphics_lib::objects::cube::Cube;
use graphics_lib::objects::object::Object;
use graphics_lib::objects::plane::Plane;
use graphics_lib::objects::poly_mesh::PolyMesh;
use graphics_lib::objects::quadratic::Quadratic;
use graphics_lib::objects::sphere::Sphere;
use graphics_lib::scene::Scene;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{fs, vec};

fn main() {
    let materials: Vec<Box<dyn Material + Sync + Send>> = vec![
        Box::new(CompoundMaterial::new_transparent_material_opacity(
            // 0 -> transparent blue
            1.05,
            Color::new(0.9, 0.7, 0.7),
            0.2,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 1 -> reflective slight blue
            Color::new(1.0, 0.3, 0.2),
            0.8,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 2 -> Red shiny
            Color::new(1., 0.3, 0.8) * 0.8,
            0.5,
        )),
        Box::new(
            CompoundMaterial::new_transparent_material(1.1), // 3 -> Transparent
        ),
        Box::new(CompoundMaterial::new_matte_material(
            // 4 -> Matte blue
            Color::new(0.4, 0.4, 0.6),
            0.2,
        )),
        Box::new(CompoundMaterial::new_reflective_material(
            // 5 -> dark slightly reflective
            Color::new(1.2, 1., 0.4) * 0.8,
            0.8,
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
            Color::new(0.4, 0.5, 0.6),
            0.4,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 10 -> green matte
            Color::new(0.4, 0.4, 0.4),
            0.4,
        )),
        Box::new(CompoundMaterial::new_matte_material(
            // 11 -> purple matte
            Color::new(0.3, 0.6, 0.3),
            0.3,
        )),
        Box::new(FalseColorMaterial::new(0.3)), // 12 -> False color
        Box::new(CompoundMaterial::new(
            vec![
                (Box::new(FalseColorMaterial::new(0.8)), 0.5),
                (
                    Box::new(CompoundMaterial::new_reflective_material(
                        Color::new(0.9, 0.7, 0.7),
                        0.8,
                    )),
                    0.5,
                ),
            ],
            Color::new(0.9, 0.7, 0.7),
        )), // 13 -> False color
        Box::new(CompoundMaterial::new_transparent_material_opacity(
            // 14 -> transparent blue
            1.1,
            Color::new(0.7, 0.7, 0.9),
            0.2,
        )),
        Box::new(CompoundMaterial::new_transparent_material_opacity(
            // 14 -> transparent green
            1.15,
            Color::new(0.7, 0.9, 0.7),
            0.2,
        )),
    ];

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.ppm")
        .unwrap();

    let mut castle = PolyMesh::from_file(
        BufReader::new(File::open("../castle.kcply").unwrap()),
        5,
        // 12,
        false,
        false,
    )
    .unwrap();

    castle.apply_transform(
        &(DAffine3::from_cols_array(&[
            0.05, 0.0, 0.0, //1
            0.0, 0.0, 0.05, // 3
            0.0, 0.05, 0.0, //2
            -11.0, -1.5, 2.0, // 3
        ])),
    );

    let mut hole = Quadratic::new([1., 0., 0., 0., 1.8, 0., 0., 1., 0., -110.], 9);
    hole.apply_transform(&DAffine3::from_translation(DVec3::new(1., 0., -6.)));

    let cave = CSG::new(
        Subtract,
        Box::new(Plane::new(
            DVec3::new(0., -4., -2.),
            DVec3::new(0.1, 0.4, -1.),
            4,
        )),
        Box::new(hole),
    );

    let land = Sphere::new(DVec3::new(-6., -10.2, 7.), 9.0, 11);

    let mut hole_2 = Quadratic::new([1., 0., 0., 0., 1.8, 0., 0., 1., 0., -110.], 9);
    hole_2.apply_transform(&DAffine3::from_translation(DVec3::new(1., 0., -6.)));

    let floor = CSG::new(
        Intersection,
        Box::new(hole_2),
        Box::new(Plane::new(
            DVec3::new(0., -2.5, 0.),
            DVec3::new(0., 1., 0.),
            1,
        )),
    );

    let mut teapot = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.kcply").unwrap()),
        0,
        true,
        true,
    )
    .unwrap();

    teapot.apply_transform(&DAffine3::from_scale(DVec3::new(0.9, 0.9, 0.9)));
    teapot.apply_transform(&DAffine3::from_rotation_x(-PI / 2.));
    teapot.apply_transform(&DAffine3::from_rotation_z(-PI / 16.));
    teapot.apply_transform(&DAffine3::from_rotation_y(PI / 6.));
    teapot.apply_transform(&DAffine3::from_translation(DVec3::new(3.5, -1., 9.)));

    let teapot = CSG::new(
        Intersection,
        Box::new(teapot),
        Box::new(Sphere::new(DVec3::new(5., 1., 10.), 100., 0)),
    );

    let mut teapot2 = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.kcply").unwrap()),
        14,
        true,
        true,
    )
    .unwrap();

    teapot2.apply_transform(&DAffine3::from_scale(DVec3::new(0.4, 0.4, 0.4)));
    teapot2.apply_transform(&DAffine3::from_rotation_x(-PI / 2.));
    teapot2.apply_transform(&DAffine3::from_rotation_z(-PI / 16.));
    teapot2.apply_transform(&DAffine3::from_rotation_y(2. * PI / 6.));
    teapot2.apply_transform(&DAffine3::from_translation(DVec3::new(4., -1.0, 4.)));

    let teapot2 = CSG::new(
        Intersection,
        Box::new(teapot2),
        Box::new(Sphere::new(DVec3::new(5.4, 1., 6.), 100., 0)),
    );

    let mut teapot3 = PolyMesh::from_file(
        BufReader::new(File::open("../teapot_smaller.kcply").unwrap()),
        15,
        true,
        true,
    )
    .unwrap();

    teapot3.apply_transform(&DAffine3::from_scale(DVec3::new(0.6, 0.6, 0.6)));
    teapot3.apply_transform(&DAffine3::from_rotation_x(-PI / 2.));
    teapot3.apply_transform(&DAffine3::from_rotation_z(-PI / 16.));
    teapot3.apply_transform(&DAffine3::from_rotation_y(-PI / 6.));
    teapot3.apply_transform(&DAffine3::from_translation(DVec3::new(2., 1.5, 10.)));

    let teapot3 = CSG::new(
        Intersection,
        Box::new(teapot3),
        Box::new(Sphere::new(DVec3::new(5., 1., 6.), 100., 0)),
    );

    // let sphere = Sphere::new(DVec3::new(4., 1., 10.), 1.5, 0);
    // let sphere2 = Sphere::new(DVec3::new(0., 1., 10.), 1.5, 12);

    let main_light = PointLight::new(DVec3::new(1.0, 10., -15.0), Color::new(0.9, 0.9, 0.9));
    let inner_light = PointLight::new(DVec3::new(0.05, 4.25, 6.05), Color::new(0.6, 0.55, 0.55));

    // let sphere = Sphere::new(DVec3::new(-2., 6., 4.), 1.2, 1);

    let mut sceptre = Quadratic::new([1., 0., 0., 0., -0.09, 0., 0., 1., 0., -0.1], 2);
    sceptre.apply_transform(&DAffine3::from_translation(DVec3::new(0., 0.0, -6.)));

    let top = Sphere::new(DVec3::new(0., 3.5, 6.), 1.4, 2);

    let sceptre = CSG::new(Subtract, Box::new(sceptre), Box::new(top));

    let bound = Plane::new(DVec3::new(0., 3.5, 6.), DVec3::new(0., 1., 0.), 2);

    let sceptre = CSG::new(Intersection, Box::new(sceptre), Box::new(bound));

    let ball = Sphere::new(DVec3::new(0., 3.5, 6.), 0.8, 13);

    let sceptre = CSG::new(Union, Box::new(sceptre), Box::new(ball));

    let scene = Scene::new(
        vec![
            Box::new(castle),
            Box::new(teapot),
            Box::new(teapot2),
            Box::new(teapot3),
            // Box::new(sphere),
            // Box::new(cube),
            Box::new(cave),
            Box::new(floor),
            Box::new(land),
            Box::new(sceptre),
        ],
        vec![
            Box::new(main_light), //
            Box::new(inner_light),
        ],
        materials,
        // Box::new(NormalCamera::new(
        //     DVec3::new(0.5, 4., -15.),
        //     DVec3::new(-0.05, -0.2, 1.0),
        //     DVec3::new(0., 1., 0.),
        //     1.4,
        // )),
        Box::new(DoFCamera::new(
            DVec3::new(0.5, 4., -15.),
            DVec3::new(-0.05, -0.2, 1.0),
            DVec3::new(0., 1., 0.),
            1.45,
            100,
            21.,
            0.2,
        )),
    );

    let fb = scene.render(1000, 1000);
    // let fb = scene.render(400, 300);

    File::write_all(&mut file, &(fb.to_rgb_file(0.4))).unwrap();
}
