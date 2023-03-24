use std::f64::consts::PI;

pub trait Area {
    fn getArea(&self) -> f64;
}

struct Rectangle {
    length:f64,
    width:f64,
}

impl Area for Rectangle {
    fn getArea(&self) -> f64 {
        self.length * self.width
    }
}

struct Circle {
    radius:f64,
}

impl Area for Circle {
    fn getArea(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Triangle {
    a: f64,
    h: f64,
}

impl Area for Triangle {
    fn getArea(&self) -> f64 {
        0.5 * self.a * self.h
    }
}

fn getArea<T: Area>(category: T) -> f64 {
    category.getArea()
}

fn main() {
    let a = Rectangle{length: 1.0, width: 2.0};
    let b = Circle{radius: 3.0};
    let c = Triangle{a: 4.0, h: 5.0};
    println!("a area {}", getArea(a));
    println!("b area {}", getArea(b));
    println!("c area {}", getArea(c));
}