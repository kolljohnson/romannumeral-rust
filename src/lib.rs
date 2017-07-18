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

    #[test]
    fn numeral_four_returns_number_four() {
        assert_eq!(4, arabic_convert("IV"));
    }

    #[test]
    fn numeral_five_returns_number_five() {
        assert_eq!(5, arabic_convert("V"));
    }

    #[test]
    fn add_one_to_one_returns_numeral_two() {
        assert_eq!("II", roman_calculator("I", "I", Operator::Add));
    }

    #[test]
    fn add_two_to_one_returns_numeral_three() {
        assert_eq!("III", roman_calculator("II", "I", Operator::Add));
    }

    #[test]
    fn add_three_to_two_returns_numeral_five() {
        assert_eq!("V", roman_calculator("II", "III", Operator::Add));
    }

    #[test]
    fn add_one_to_three_returns_numeral_four() {
        assert_eq!("IV", roman_calculator("I", "III", Operator::Add));
    }

    #[test]
    fn subtract_twenty_five_to_eight_returns_numeral_seventeen() {
        assert_eq!("XVII", roman_calculator("XXV", "VIII", Operator::Subtract));
    }

    #[test]
    fn multiply_five_to_five_returns_numeral_twenty_five() {
        assert_eq!("XXV", roman_calculator("V", "V", Operator::Multiply));
    }

    #[test]
    fn divide_thirty_nine_by_thirteen_returns_numeral_three() {
        assert_eq!("III", roman_calculator("XXXIX", "XIII", Operator::Divide));
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

fn arabic_convert(numeral: &str) -> u32 {
    match NUMERAL_LIST.iter()
        .find( |val| numeral.starts_with(val.numeral)) {
            Some(val) => val.value + arabic_convert(&numeral[val.numeral.len()..]),
            None => 0
        }
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn roman_calculator(first_numeral: &str, second_numeral: &str, operator: Operator) -> String {
    let first_number = arabic_convert(first_numeral);
    let second_number = arabic_convert(second_numeral);
    match operator {
        Operator::Add => roman_convert(first_number + second_number),
        Operator::Subtract => roman_convert(first_number - second_number),
        Operator::Multiply => roman_convert(first_number * second_number),
        Operator::Divide => roman_convert(first_number / second_number),
        _ => String::from("Operator not supported.")
    }
}

