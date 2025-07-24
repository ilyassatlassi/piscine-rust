use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius: radius,
        }
    }
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2) as f64
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64);
