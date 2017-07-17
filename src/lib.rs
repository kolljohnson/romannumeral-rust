#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
    }

    #[test]
    fn number_one_returns_numeral_one() {
        assert_eq!("I", roman_convert(1));
    }

    #[test]
    fn number_two_returns_numeral_two() {
        assert_eq!("II", roman_convert(2));
    }
}

fn roman_convert(number: i32) -> String {
    let mut numeral = String::new();

    for x in 0..number {
        numeral.push_str("I");
    }
    numeral
}

