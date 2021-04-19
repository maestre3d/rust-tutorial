fn main() {
    // Immutable by default (constants)
    let a_number = 10;
    let is_exampple = true;

    println!("the number is {}", a_number);
    println!("the flag is {}", is_exampple);

    // Mutable data (variable)
    let mut var_number = 5;
    println!("the number is {}", var_number);
    var_number += 10; // using variable shadowing
    println!("now the number is {}", var_number);

    run_data_types_example()
}

/**
 * Private func which executes ops with specific data types
 */
fn run_data_types_example() {
    let x : u32 = 10;
    let y : f64 = 2.501;

    println!("the numbers with data types are {} and {}", x, y);

    println!("1 + 9 = {}", 1u32 + 9);

    let is_bigger : bool = 2.499f64 > y;

    println!("{}", is_bigger);

    let natural_char: char = 'ℤ'; // Rust's chars are UTF-8 encoded and 32 bit wide by default.

    println!("{}", natural_char);

    let mut hello = String::from("Hello, "); // Rust String are char slices
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", hello);

    let tuple = ("hello", 5, 'z');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'z');

    let arr = ["hello", "there"]; // arrays are fixed in length
    println!("{} {}", arr[0], arr[1]);

    let mut vec = Vec::new(); // a vector can grow or shrink anytime
    vec.push("hello");
    vec.push("there from vector");

    println!("{} {}", vec[0], vec[1]);
}
