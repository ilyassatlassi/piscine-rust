mod areas_volumes;

use crate::areas_volumes::*;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    match kind{
        GeometricalShapes::Rectangle => {
             x * y > areas_volumes::rectangle_area(a, b) * times
        },
        _ => false,
    }
    // todo!()
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    todo!()
}
