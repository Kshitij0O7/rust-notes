// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main(){
//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));

//     println!("{home:?} {loopback:?}");
// }

#[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
/* The same could be written as given below but then for each of them we would need a different function
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

*/

// Enums also have option to have their own methods with impl keyword as shown below and is refered as &self in these
// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// fn main(){
//     // let message = Message::Quit;
//     printMessage(Message::Quit);
//     printMessage(Message::Move{x:23, y:24});
//     printMessage(Message::Write(String::from("Hello")));
//     printMessage(Message::ChangeColor(0,1,3));
// }
// // Enums would let you create a single function that handles different types of values as shown below
// fn printMessage(message: Message){
//     println!("{message:?}");
// }


// Option<T> Enum - way of rust to implement null

enum Option<T> {
    None,
    Some(T),
}

fn main(){
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{some_number:?} {some_char:?} {absent_number:?}");
}
