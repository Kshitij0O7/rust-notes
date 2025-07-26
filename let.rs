// Notes on if let and let else

// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {max}"),
//         _ => (),
//     }
// }

// Handles for Some(u8) value and None value as well

// The above could be simplified as shown below
// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest

// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {max}");
//     }
// }

// Handles only for Some(u8) value and ignores the handling for None altogether

/*
Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. 
Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate 
trade-off for losing exhaustive checking.
*/

// else with if let

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

// The above could be rewritten as

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin { // storing in variable to not run out of scope
//         state
//     } else {
//         return None;
//     };

//     if state.existed_in(1900) {
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

// To further simplify we can use let else

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    // let coin = Coin::Quarter(UsState::Alaska);
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }
    // The above could be rewritten as below:
    // let coin = Coin::Penny;
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {state:?}!");
    // } else {
    //     count += 1;
    //     println!("{count}");
    // }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        println!("{desc}");
    }
}