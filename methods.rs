// Methods are similar to fns but defined in the context of structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are wrapped inside this block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Methods in imp as constructor exapmle 
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// Note -> A single struct can have multiple imp blocks
// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// For calling methods on object Rust has a feature called automatic referencing and dereferencing