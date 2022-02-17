pub fn main() {
    // Rust does type inference by default, but you can explicitly specify types if you want
    // Basic data types are all max 3 characters, typically three
    //
    // Numeric types:      i8, i16, i32, i64, i128,
    //                     u8, u16, u32, u64, u128,
    //                     f32, f64
    // Logic types:        bool
    // Alphanumeric types: char, str

    let first_num = 1;
    let second_num: i8 = 2;
    let third_num: f32 = 20.5;

    // Variables are immutable by default, but can be made mutable with the "mut" keyword
    // This implies deep immutability, i.e. even collections cannot be mutated without "mut"
    let mut result = 0;
    result = first_num + second_num as i32; // There are no implicit casts either!
}