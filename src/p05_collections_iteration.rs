pub fn main() {
    // Vectors are similar to Java's ArrayLists. There are other classic data structures in the lanugage like sets, queues, etc. too
    let favorite_numbers = vec![1337, 9001];

    // You can iterate over vectors by creating iterators
    for favorite_number in favorite_numbers.iter() {
        println!("One of my favorite numbers is {}!", favorite_number);
    }

    // Your favorite collection operations are supported too, you just need to make an iterator for them first
    let (over_5000, under_5000): (Vec<i32>, Vec<i32>) = favorite_numbers.iter().partition(|&number| *number > 5000);
    println!("Favorite numbers under 5000: {:?}, over 5000: {:?}", under_5000, over_5000);
    let favorite_total: i32 = favorite_numbers.iter().sum();
    println!("The total of all my favorite numbers is {}", favorite_total);

    // Rust supports tuples & tuple destructuring if you like tuples:
    let my_tuple = (5, 6);
    let (num1, num2) = my_tuple;
    println!("Num1 is {} and num2 is {}", num1, num2);
}