// pub fn twice<T>(F: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
//     move |x| F(F(x))

// }
pub fn twice<T:  Fn(i32) -> i32>(f: T) -> impl Fn(i32) -> i32 {
    move |x| f(f(x))
}
pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
