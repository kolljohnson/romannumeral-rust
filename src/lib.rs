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

    #[test]
    fn number_four_returns_numeral_four() {
        assert_eq!("IV", roman_convert(4));
    }

    #[test]
    fn number_five_returns_numeral_five() {
        assert_eq!("V", roman_convert(5));
    }
}

struct RomanNumeral {
    numeral: &'static str,
    value: u32
}

const NUMERAL_LIST: [RomanNumeral; 3] = [
    RomanNumeral {numeral: "V", value: 5},
    RomanNumeral {numeral: "IV", value: 4},
    RomanNumeral {numeral: "I", value: 1}
];

fn roman_convert(mut number: u32) -> String {
    let mut converted = String::new();

    for numeral in NUMERAL_LIST.iter() {
        while numeral.value <= number {
            converted = converted + numeral.numeral;
            number -= numeral.value; 
        } 
    }

    converted
}

