// use std::{f32::consts::PI, f64::consts::PI};
use std::f64::consts::PI;
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
        PI * self.radius.powi(2)
    }
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    pub fn intersect(&self, circle2: Circle) -> bool {
        let distnace = self.center.distance(circle2.center);
        if self.radius + circle2.radius >= distnace {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);
impl Point {
    pub fn distance(&self, point2: Point) -> f64 {
        let dx = self.0 - point2.0;
        let dy = self.1 - point2.1;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_new_circle() {
        let circle = Circle::new(500.0, 400.0, 150.0);
        assert!(approx_eq(circle.radius, 150.0));
        assert!(approx_eq(circle.center.0, 500.0));
        assert!(approx_eq(circle.center.1, 400.0));
    }

    #[test]
    fn test_distance() {
        let a = Point(0.0, 1.0);
        let b = Point(0.0, 0.0);
        assert!(approx_eq(a.distance(b), 1.0));

        let a = Point(1.0, 0.0);
        let b = Point(0.0, 0.0);
        assert!(approx_eq(a.distance(b), 1.0));

        let a = Point(1.0, 1.0);
        let b = Point(0.0, 0.0);
        assert!(approx_eq(a.distance(b), f64::sqrt(2.0)));
    }

    #[test]
    fn test_area() {
        let circle = Circle::new(500.0, 400.0, 150.0);
        assert!(approx_eq(circle.area(), 70685.83470577035));
    }

    #[test]
    fn test_intersection() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle::new(80.0, 115.0, 30.0);
        assert!(!circle.intersect(circle1));

        let circle = Circle::new(100.0, 300.0, 150.0);
        let circle1 = Circle::new(80.0, 115.0, 100.0);
        assert!(circle.intersect(circle1));
    }
}
