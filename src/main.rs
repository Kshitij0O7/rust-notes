// fn main() {
//     let x: i8 = -1; -> num next to i determines size of variable stored, can be both +ve and -ve
//     let y: u8 = 3; -> can be only +ve
//     let z: f32 = 100.001; -> f makes it float enabling decimal
//     let x = 1 -> type by default is i32
//     println!("{}, {}, {}", x, y, z);
// }

// fn main() {
//     let is_male = true;
//     let is_above_18 = false;
    
//     if is_male {
//         println!("You are a male");

//     } else {
//         println!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal male");
//     }
// }

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

fn main() {
    // Tupple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("{}, {}, {}", x, y, z); 
    // ***Can't print tuple like -> println!("{}", tup);*** //
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let x = tup.0;
    // println!("{}", x); -> output = 500

    // Array
    let a = [1, 2, 3, 4, 5];
    println!("{}",a)

    // let mut spaces = "   ";
    // spaces = spaces.len();

    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("{}", spaces);

    /*
    The first spaces variable is a string type and the second spaces variable is a number type. 
    Shadowing thus spares us from having to come up with different names, such as spaces_str
    and spaces_num; instead, we can reuse the simpler spaces name. 
    However, if we try to use mut for this, as shown here, we’ll get a compile-time error
     */
}