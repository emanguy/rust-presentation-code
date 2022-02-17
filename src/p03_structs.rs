// There are no objects in rust, but you can make data structures with struct
pub struct Person {
    first_name: String,
    last_name: String,
}

// Structs can have associated methods via implementation blocks
impl Person {
    // This is a "member function" which uses the struct's data
    pub fn full_name(&self) -> String {
        // Like functional languages, the last expression in a function is the return value
        // You can still use "return" if you want though
        format!("{} {}", self.first_name, self.last_name)
    }

    // This is more similar to a static function because "self" isn't specified
    pub fn new_john_doe() -> Person {
        return Person { first_name: String::from("John"), last_name: String::from("Doe")};
    }
}

pub fn main() {
    let jdoe = Person::new_john_doe();
    println!("{}", jdoe.full_name());

    // Rust supports destructuring on struct types (as well as tuples and some collection types)
    let Person { first_name, .. } = jdoe;
    println!("John doe's first name is {}", first_name);
}