use std::fmt::{Display, Formatter};
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

// Additionally, you can implement traits for external types, making it easy to extend
// the functionality of existing parts of the language
impl Magnitude for i32 {
    fn magnitude(self) -> f64 {
        self as f64
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
        Point { 
            x_pos: self.x_pos, 
            y_pos: self.y_pos,
         }
    }
}

// ...or generated via the derive() macro. JSON libraries use this, for example.
#[derive(Clone)]
struct ClonablePoint {
    x_pos: i32,
    y_pos: i32
}

// Because traits are like interfaces, you can accept any type that implements a trait as a parameter to functions
fn magnitude_of(with_magnitude: impl Magnitude) -> f64 {
    with_magnitude.magnitude()
}

// You can also implement traits generically. In this case, we can implement a "double print" trait
// on anything that implements Display (sort of like "to string" in other languages)
trait DoublePrint {
    fn double_print(&self) -> String;
}

impl<T: Display> DoublePrint for T {
    fn double_print(&self) -> String {
        format!("{} {}", self, self)
    }
}

//...for this we also need a "display" implementation for point so let's do that:
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x_pos, self.y_pos)
    }
}

pub fn main() {
    let point1 = Point { x_pos: 2, y_pos: 1 };
    let point2 = point1.clone();

    let point3 = &point1 + &point2;
    println!("The magnitude of the sum of the points is {}.", magnitude_of(&point3));

    String::yeehaw();
    println!("Point 1 doubled: {} number 5 doubled: {}", point1.double_print(), 5.double_print())
}