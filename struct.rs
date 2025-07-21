// Example struct (similar to langs like JS)
/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
*/
#[derive(Debug)] // This needs to be added just line that one line mentioning license in solidity
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() {
//     let mut user1 = build_user(String::from("someemail@example.com"), String::from("abg"));

//     println!("{:?}", user1);

//     // user1.email = String::from("anotheremail@example.com");
    
//     // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable

//     // println!("{:?}", user1);
//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("other@example.com"),
//         sign_in_count: user1.sign_in_count,
//     }; // An example of struct update syntax. 

//     println!("{:?}", user2);

//     // Struct update syntax with ..
//     let user3 = User{
//         email: String::from("another@example.com"),
//         ..user2
//     };
//     println!("{:?}", user3);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// Tuple like structs -> useful when a struct don’t require to have names associated with their fields

// #[derive(Debug)]
// struct Color(i32, i32, i32);
// #[derive(Debug)] // This has to be added above every struct
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("{:?} {:?}", black, origin);
// }

// Unit-Like Structs Without Any Fields -> Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

#[derive(Debug)]
struct AlwaysEqual; // Basically a characterstick checkbox

fn main() {
    let subject = AlwaysEqual;
    println!("{:?}", subject);
}
