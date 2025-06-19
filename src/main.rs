// fn main() {
//     let x: i8 = -1; -> num next to i determines size of variable stored, can be both +ve and -ve
//     let y: u8 = 3; -> can be only +ve
//     let z: f32 = 100.001; -> f makes it float enabling decimal
//     let x = 1 -> type by default is i32
//     println!("{}, {}, {}", x, y, z);
// }

fn main() {
    let is_male = true;
    let is_above_18 = false;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }
}

// fn main() {
//     let greeting = String::from("hello world");
//     println!("{}", greeting);
//     // let mut test = "test";
//     // test = "werfgvcdxfgbvc";
//     // println!("{}", test);
//     let char = greeting.chars().nth(1); // This is optionaly character i.e may or may not b character
//     print!("{}", char.unwrap()) // will work if it exist, will throw error if it doesn't
//     // Problem with strings -> no fixed type like array/vector.
//     // They change space at runtime, RAM allocates just enogh space to fit it and on update it would again need memory allotment making operation slow and unsafe
// }




// fn main() {
//     let mut x: i8 = 2; -> to modify variable it has to be defined as mutable
//     println!("{}", x);
//     x += 10;
//     println!("{}", x);
// }

// * compiler can only return static errors and not run time errors

