pub fn main() {
    // Enums are immensely valuable in Rust because they power the Option and Result types

    // The option enum allows you to express a value that may or may not be there
    let value_exists = Some(5);
    let value_doesnt_exist: Option<i32> = None;

    let acquired_value = match value_exists {
        Some(value) => value,
        None => 0,
    };
    // Or more succinctly...
    let easier_value_get = value_doesnt_exist.unwrap_or(0);

    // The option type is also used with iterators
    // Here's how you could double a list of numbers with an iterator
    let mut numbers = vec![1, 2, 3];
    let mut number_iterator = numbers.iter_mut(); // iter_mut produces an iterator that lets you mutate the values
    while let Some(number) = number_iterator.next() {
        *number = *number * 2;
    }
    println!("The doubled list of numbers is {:?}", numbers);

    // Let's try using our database_lookup function...
    if let Ok(sum) = database_lookup_v2() {
        println!("The sum of all our product prices is {}", sum);
    } else {
        println!("Oops, something went wrong while checking the DB!");
    }
}

enum DbError {
    ConnectionProblem,
    ViolatedConstraint,
}

fn database_operation_fails() -> Result<Vec<i32>, DbError> {
    // Do something...
    Err(DbError::ConnectionProblem)
}

fn database_lookup_v1() -> Result<i32, DbError> {
    // Now we can perform a database operation and match on whether or not it succeeded
    let db_result = match database_operation_fails() {
        Ok(result_list) => result_list,
        Err(db_error) => return Err(db_error),
    };

    Ok(db_result.iter().sum())
}

fn database_lookup_v2() -> Result<i32, DbError> {
    // The match pattern in v1 is so common that Rust lets you immediately return if
    // a result is a Result::Err using the question mark operator
    //
    // The question mark operator can also be used with the Option enum if your function returns
    // an Option value
    let total = database_operation_fails()?
        .iter()
        .sum();

    Ok(total)
}
