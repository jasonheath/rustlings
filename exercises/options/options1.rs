// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_ice_cream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22
    // The Option output should gracefully handle cases where time_of_day > 24.
    match time_of_day {
        0..=21 => Some(5),
        22..=24 => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(10), Some(5));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(25), None);
    }

    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12).unwrap_or(255);
        assert_eq!(ice_creams, 5);
    }
}
