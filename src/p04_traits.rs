use std::ops::Add;

// Traits are like interfaces in other languages, defining behavior that will exist
trait Magnitude {
    fn magnitude(self) -> f64;
}

struct Point {
    x_pos: i32,
    y_pos: i32,
}

// You create impl blocks for each trait to implement it for your struct
impl Magnitude for &Point {
    fn magnitude(self) -> f64 {
        (self.x_pos.pow(2) as f64 + self.y_pos.pow(2) as f64).sqrt()
    }
}

// Traits can be used for operator overloading
impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x_pos: self.x_pos + other.x_pos,
            y_pos: self.y_pos + other.y_pos
        }
    }
}

// Some traits can be implemented manually...
impl Clone for Point {
    fn clone(&self) -> Self {
        Self { x_pos: self.x_pos.clone(), y_pos: self.y_pos.clone() }
    }
}

// ...or generated via the derive() macro. JSON libraries use this, for example.
#[derive(Clone)]
struct ClonablePoint {
    x_pos: i32,
    y_pos: i32
}

// Because traits are like interfaces, you can accept any type that implements an interface as a parameter to functions
fn magnitude_of(with_magnitude: impl Magnitude) -> f64 {
    with_magnitude.magnitude()
}

pub fn main() {
    let point1 = Point { x_pos: 2, y_pos: 1 };
    let point2 = point1.clone();

    let point3 = &point1 + &point2;
    println!("The magnitude of the sum of the points is {}.", magnitude_of(&point3));
}