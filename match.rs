// Match -> Extremely powerful control flow construct that allows you to compare a value against a series of patterns and then execute code based on which pattern matches
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         },
//     }
// }

// fn main(){
//     println!("{}", value_in_cents(Coin::Penny));
//     println!("{}", value_in_cents(Coin::Nickel));
//     println!("{}", value_in_cents(Coin::Dime));
//     println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
// }

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern

// Matching with Option<T>

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{five:?} {six:?} {none:?}"); // These Option<T> requires this aswell - `:?`
// }

// There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities

// Possible fix -> use `others`

// fn main(){
//     let dice_roll = 3;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }

    // fn add_fancy_hat() {println!("You got a hat")}
    // fn remove_fancy_hat() {println!("Your hat is taken")}
    // fn move_player(num_spaces: u8) {println!("{num_spaces}")}

// }

// Note that we have to put the catch-all arm last because the patterns are evaluated in order. If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!

// Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

fn main(){
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {println!("You got a hat");}
    fn remove_fancy_hat() {println!("Your hat is taken");}
    // fn reroll() {println!("You got a reroll");}
}