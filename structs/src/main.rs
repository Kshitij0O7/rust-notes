// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(rect1); // dbg!(&rect1) is a debugging tool that helps to print expression value
    // dgb returns the ownership of value of expression, but not the same with rect hence ref is passed

    // println!("rect1 is {rect1:#?}"); // :? -> used to display array or struct; :#? -> same as before but prettified
    // Place value before the symbols if you want to put it inside "{}"
}