pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut min = nb_1;

    let mut max = nb_1;
    if min > nb_2 {
        min = nb_2;
    }
    if min > nb_3 {
        min = nb_3;
    }
    if max < nb_2 {
        max = nb_2;
    }
    if max < nb_3 {
        max = nb_3;
    }
    (min, max)
}
