// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s); // can't modify something you dont have ownership of
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s); // we can modify without passing ownership by passing mut ref?? --> yes, but if a variable has mut ref it cannot have any another mut ref at the same time
//     println!("{s}"); // Also if it is already borrowed as a immut ref then we can have other immut ref but no mut ref
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.

// Code expected to create a dangling reference

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// This will not work as if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.


// Slice -> Lets you refer an element in collection rather than whole

// Getting index without using slice
// fn main(){
//     let s = String::from("hello world");
//     println!("{}", first_word(&s));
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); // Convert string to bytes array

//     for (i, &item) in bytes.iter().enumerate() { // byte array iterator that returns each element as part of a tuple instead
//         if item == b' ' { // search for the mentioned element inside the said tuple
//             return i;
//         }
//     }

//     s.len()
// } 

// This is a very problematic approach so we introduce slice now

// fn main(){
//     let s = String::from("hello world");

//     // let hello = &s[0..5]; // reference to the part of string from index 0 to 5
//     let hello = &s[..5]; // Above could be rewritten as this because the index starts from 0
//     let world = &s[6..11]; // reference to the part of string from index 6 to 11
//     // Drop both values if you need entore string -> let str = &s[..]

//     println!("{} {}", hello, world);
// }

/*
Internally, the slice data structure stores the starting position and 
the length of the slice, which corresponds to ending_index minus starting_index 
*/

// Rewritten first_word function with slice in mind

// fn main(){
//     let mut s = String::from("Hello World");
//     let len = s.len();

//     let word = first_word(&s);
//     let end = second_word(&s, len);

//     println!("the first word is: {word}\nthe second word is: {end}");
//     s.clear();
// }

// fn first_word(s: &str) -> &str {// the parameter is a slice ref now str denotes slice
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// fn second_word(s: &str, len:usize) -> &str{
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[(i+1)..len];
//         }
//     }

//     &s[..]
// }

// Slices in context to others like array

fn main(){
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    // assert_eq!(slice, &[2, 3]);
    println!("Complete Array:{:?}\nPart of it:{:?}", a, slice);
}