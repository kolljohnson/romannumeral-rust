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
}

fn roman_convert(number: i32) -> String {
    let numeral = String::from("I");
    numeral
}

