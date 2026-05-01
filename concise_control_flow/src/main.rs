#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // instead of match we can use `if let`
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    // match coin {
        
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    let coin_two = Coin::Quarter(UsState::Alabama);

    let coin_result = describe_state_quarter(coin_two);

    if let Some(coin_result) = coin_result {
        println!("{coin_result}");
    } else {
        println!("Not a quarter!");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for America!"))
    //     } else {
    //         Some(format!("{state:?} is relatively new."))
    //     }
    // } else {
    //     None
    // }

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    // if state.existed_in(1900) {
    //     Some(format!("{state:?} is pretty old, for America!"))
    // } else {
    //     Some(format!("{state:?} is relatively new."))
    // }

    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
