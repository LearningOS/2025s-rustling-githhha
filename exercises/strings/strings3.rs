// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    //  Remove whitespace from both ends of a string!
    let mut start = 0;
    let mut end = input.len();


    while start < input.len() && input.chars().nth(start).unwrap().is_whitespace() {
        start += 1;
    }
    while end > 0 && input.chars().nth(end - 1).unwrap().is_whitespace() {
        end -= 1;
    }
    input[start..end].to_string()
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There's multiple ways to do this!
    let mut a = input.to_string();
    a.push_str(" world!");
    a
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    input.to_string().replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
