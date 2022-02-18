use std::fmt::Display;

pub fn main() {
    let my_number = 5;

    // Match statements are like enhanced switch statements and can "pattern match" values:
    match my_number {
        1 => println!("Your number is 1!"),
        2 => println!("Your number is 2!"),
        3 => println!("Your number is 3!"),
        4..=5 => println!("Your number is in the range of 4-5!"),
        6..=10 => println!("Your number is in the range of 6-10!"),
        _ => println!("I don't know what your number is..."),
    };

    // All statements in rust (except loops) are expressions, so we can use match statements to return values:
    let new_number = match my_number {
        1..=5 => 1,
        6 | 7 | 8 => 6,
        9..=20 => 9,
        _ => 21,
    };
    println!("Your new number is {}", new_number);

    // Make sure you check the definition for this color enum!
    let my_favorite_color = Color::Green;
    let your_favorite_color = Color::RGB(44, 53, 212);

    println!("My favorite color is {} and yours is {}.", my_favorite_color, your_favorite_color);

    // You can also pattern match with an if statement
    // Here, we verify your_favorite_color is a Color::RGB and then destructure the first value of its tuple into red_value
    if let Color::RGB(red_value, ..) = your_favorite_color {
        println!("Your favorite color's red value is {}", red_value);
    } else {
        println!("Your favorite color isn't an RGB value.");
    }
}

// Rust enums behave like typical enums but they can hold data
enum Color {
    Red,
    Green,
    Blue,
    RGB(i32, i32, i32), // This enum value holds a tuple of 3 numbers
    RGBA { // This enum value holds values like a struct
        r: i32, 
        g: i32, 
        b: i32, 
        a: i32,
    }, 
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We can use match statements to pattern match against enums:
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),

            Color::RGB(r, g, b) => write!(f, "(R: {} G: {} B: {})", r, g, b),

            Color::RGBA { r: red, g: green, b: blue, a: alpha} => 
                write!(f, "(R: {}, G: {}, B: {}, A: {})", red, blue, green, alpha),
        }
    }
}