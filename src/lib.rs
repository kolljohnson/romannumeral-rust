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

    #[test]
    fn numeral_one_returns_number_one() {
        assert_eq!(1, arabic_convert("I"));
    }

    #[test]
    fn numeral_two_returns_number_two() {
        assert_eq!(2, arabic_convert("II"));
    }
}

struct RomanNumeral {
    numeral: &'static str,
    value: u32
}

const NUMERAL_LIST: [RomanNumeral; 13] = [
    RomanNumeral {numeral: "M", value: 1000},
    RomanNumeral {numeral: "CM", value: 900},
    RomanNumeral {numeral: "D", value: 500},
    RomanNumeral {numeral: "CD", value: 400},
    RomanNumeral {numeral: "C", value: 100},
    RomanNumeral {numeral: "XC", value: 90},
    RomanNumeral {numeral: "L", value: 50},
    RomanNumeral {numeral: "XL", value: 40},
    RomanNumeral {numeral: "X", value: 10},
    RomanNumeral {numeral: "IX", value: 9},
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

fn arabic_convert(mut numeral: &str) -> u32 {
    let mut converted = 0;

    for character in numeral.chars() {
        converted = converted + 1; 
    }
    converted
}

