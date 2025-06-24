// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }
// Statements and Expressions
// Statements are instructions that perform some action and do not return a value -> let y = 6;
// Expressions evaluate to a resultant value ->  

// fn main() {
//     let y = {let x = 3; x + 1}; // This is an expression -> { let x = 3; x + 1}

// // Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons

//     println!("The value of y is: {y}");
// }

// fn five() -> i32 {
//     10
// }

// fn main() {
//     let x = five(); // Equivalent to let x = 5;

//     println!("The value of x is: {x}");
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/* 
We define a function in Rust by entering fn followed by a function name and a 
set of parentheses. The curly brackets tell the compiler where the function body 
begins and end.
 */ 