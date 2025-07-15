fn main(){
    // let mut s = "swgffhy"; -> string literal
    // println!("{s}");
    // s = &(s.to_owned()+"1234567"); -> concatenation is almost impossible
    // println!("{s}");

    // let mut s = String::from("hello");
    // s.push_str(", world!"); // concatenation is quite easy
    // println!("{s}");

    // let s1 = String::from("hello");
    // let s2 = s1;
    // This is a case of shallow copy, s2 points to the same memory unit as s1
    // s1 is created and stored on stack and has info such as pointer and length for the String stored on heap
    // with let s2 = s1, we are making go out of scope after s1 is assigned to s2, -> a concept of ownership that manages garbage space removal
    // println!("{s2}");
    
    // If we do want to deeply copy the heap data of the String, then following should be used
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // New stack object s2 is created that points to a cloned heap that s1 was pointing to
    println!("s1 = {s1}, s2 = {s2}");
}