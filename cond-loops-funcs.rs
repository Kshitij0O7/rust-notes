// pub fn main() {
//     let x = 99;
//     let is_even = is_even(x);
//     if is_even {
//         print!("{} is even", x);
//     } else {
//         print!("{} is odd", x);
//     }
// }

// pub fn is_even(x: i32) -> bool {
//     return x % 2 == 0;
// }

pub fn main() {
    let str = String::from("harkirat singh");
    println!("First name {}", get_first_name(str));

    // for _i in 0..100{ // addition is already taken care of i++ is auto implemented
    //     println!("Hello World!");
    // }
    // If not using the variable anywhere/i is just iterator add _ as prifix
    
}
// ** Strings are stored in heaps in a byte after byte manner
pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}
