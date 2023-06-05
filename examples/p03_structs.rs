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

    // Just so you're aware, putting an ampersand before something creates a reference to it
    // For demonstration, we'll create a mutable reference
    let mut jdoe_2 = Person::new_john_doe();
    let jdoe_ref = &mut jdoe_2;

    // Dereferencing gets you the original value if you need it
    (*jdoe_ref).first_name = String::from("jon");

    // You can still use functions off a reference without dereferencing
    println!("Using the reference: {}", jdoe_ref.full_name());
    
    // Since a reference just "refers" to the thing it references, you can apply changes to the
    // original value through a mutable reference. References are much smaller and easier to copy around
    // because they hold the address of the original value rather than the data
    println!("Here's what the original looks like after the fact: {}", jdoe_2.full_name());
}