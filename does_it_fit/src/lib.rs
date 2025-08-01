pub mod areas_volumes;

use crate::areas_volumes::*;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    match kind {
        GeometricalShapes::Rectangle => x * y > areas_volumes::rectangle_area(a, b) * times,

        GeometricalShapes::Triangle => {
            (x * y) as f64 > areas_volumes::triangle_area(a, b) * times as f64
        }

        GeometricalShapes::Circle => (x * y) as f64 > areas_volumes::circle_area(a) * times as f64,

        GeometricalShapes::Square => (x * y) > areas_volumes::square_area(a) * times,
    }
    // todo!()
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    match kind {
        GeometricalVolumes::Cone => {
            (x * y * z) as f64 > areas_volumes::cone_volume(a, b) * times as f64
        }

        GeometricalVolumes::Cube => (x * y * z) > areas_volumes::cube_volume(a) * times,

        GeometricalVolumes::TriangularPyramid => {
            (x * y * z) as f64
                > areas_volumes::triangular_pyramid_volume(a as f64, b) * times as f64
        }

        GeometricalVolumes::Parallelepiped => {
            (x * y * z) > areas_volumes::parallelepiped_volume(a, b, c) * times
        }

        GeometricalVolumes::Sphere => {
            (x * y * z) as f64 > areas_volumes::sphere_volume(a) * times as f64
        }
    }
    // todo!()
}
#[test]
fn no_volumes_shapes() {
    assert!(area_fit((2, 5), GeometricalShapes::Circle, 0, (2, 1)));
    assert!(area_fit((2, 2), GeometricalShapes::Rectangle, 0, (6, 10)));
    assert!(volume_fit(
        (2, 5, 3),
        GeometricalVolumes::Cone,
        0,
        (1, 1, 1)
    ));
    assert!(volume_fit(
        (3, 5, 7),
        GeometricalVolumes::Parallelepiped,
        0,
        (2, 6, 3)
    ));
}

#[test]
fn equal_size() {
    assert!(area_fit((2, 5), GeometricalShapes::Square, 1, (2, 5)));
    assert!(volume_fit(
        (3, 1, 4),
        GeometricalVolumes::Cube,
        1,
        (1, 3, 4)
    ));
}

#[test]
fn all_shapes() {
    assert!(!area_fit((3, 5), GeometricalShapes::Circle, 2, (2, 0)));
    assert!(area_fit((8, 6), GeometricalShapes::Triangle, 5, (5, 2)));
    assert!(area_fit((7, 3), GeometricalShapes::Rectangle, 2, (2, 4)));
    assert!(area_fit((5, 5), GeometricalShapes::Square, 1, (2, 4)));
}

#[test]
fn all_volumes() {
    assert!(volume_fit(
        (5, 6, 3),
        GeometricalVolumes::Cube,
        2,
        (3, 3, 4)
    ));
    assert!(!volume_fit(
        (7, 4, 4),
        GeometricalVolumes::Cone,
        1,
        (8, 2, 0)
    ));
    assert!(volume_fit(
        (2, 5, 3),
        GeometricalVolumes::Sphere,
        1,
        (1, 1, 1)
    ));
    assert!(!volume_fit(
        (2, 5, 3),
        GeometricalVolumes::Parallelepiped,
        31,
        (1, 1, 1)
    ));
    assert!(volume_fit(
        (7, 5, 3),
        GeometricalVolumes::TriangularPyramid,
        3,
        (3, 2, 1)
    ));
}