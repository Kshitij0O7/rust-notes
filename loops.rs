// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Loop Labels

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// Conditional Loops 

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// For loop to iterate collection

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// The safety and conciseness of for loops make them the most commonly used loop construct in Rust

// While loop example equivalent in for loop 

fn main() {
    for number in (1..4).rev() { // Note: The numbers mentioned are excluded
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}