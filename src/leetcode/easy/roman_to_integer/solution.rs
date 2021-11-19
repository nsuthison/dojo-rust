use once_cell::sync::Lazy;
use std::collections::HashMap;

struct Solution;

static ROMAN_NUMBER_VALUE: Lazy<HashMap<char, i32>> = Lazy::new(|| {
    HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ])
});

/// Question: https://leetcode.com/problems/roman-to-integer/
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut previous_roman_number = "".to_owned();
        let mut result = 0;
        let size = s.chars().count();

        if size == 1 {
            return get_dec_value_from_roman_number(&s).unwrap();
        }

        for (idx, roman_number_symbol) in s.chars().enumerate() {
            if previous_roman_number == "" {
                previous_roman_number = format!("{}", roman_number_symbol);
                continue;
            }

            let to_check = format!("{}{}", previous_roman_number, roman_number_symbol);

            if is_legit_roman_number(&to_check) {
                if idx < size - 1 {
                    previous_roman_number = format!("{}", to_check);
                    continue;
                } else {
                    result += get_dec_value_from_roman_number(&to_check).unwrap();
                }
            } else {
                result += get_dec_value_from_roman_number(&previous_roman_number).unwrap();
                previous_roman_number = format!("{}", roman_number_symbol);
            }
        }

        result
    }
}

fn get_dec_value_from_roman_number(roman_number: &str) -> Option<i32> {
    if !is_legit_roman_number(roman_number) {
        return None;
    }

    match roman_number.chars().count() {
        1 => Some(
            *ROMAN_NUMBER_VALUE
                .get(&roman_number.chars().nth(0).unwrap())
                .unwrap(),
        ),
        2 => {
            let first = try_get_dec_value_from_roman_nth(roman_number, 0);
            let second = try_get_dec_value_from_roman_nth(roman_number, 1);

            Some(second - first)
        }
        3 => {
            let first = try_get_dec_value_from_roman_nth(roman_number, 0);

            Some(first * 3)
        }
        _ => None,
    }
}

fn try_get_dec_value_from_roman_nth(roman_number: &str, nth: usize) -> &i32 {
    ROMAN_NUMBER_VALUE
        .get(&roman_number.chars().nth(nth).unwrap())
        .unwrap()
}

fn is_legit_roman_number(roman_number: &str) -> bool {
    return match roman_number.chars().count() {
        1 => is_legit_single_roman_number_symbol(roman_number),
        2 => is_first_roman_symbol_lower_than_the_second_one(roman_number),
        3 => are_all_three_roman_symbol_the_same(roman_number),
        _ => false,
    };
}

fn is_legit_single_roman_number_symbol(roman_number: &str) -> bool {
    if let Some(_) = ROMAN_NUMBER_VALUE.get(&roman_number.chars().nth(0).unwrap()) {
        true
    } else {
        false
    }
}

fn is_first_roman_symbol_lower_than_the_second_one(roman_number: &str) -> bool {
    let first = try_get_dec_value_from_roman_nth(roman_number, 0);
    let second = try_get_dec_value_from_roman_nth(roman_number, 1);

    first <= second
}

fn are_all_three_roman_symbol_the_same(roman_number: &str) -> bool {
    let first = try_get_dec_value_from_roman_nth(roman_number, 0);
    let second = try_get_dec_value_from_roman_nth(roman_number, 1);
    let third = try_get_dec_value_from_roman_nth(roman_number, 2);

    (first == second) && (first == third)
}

#[cfg(test)]
pub mod solution_test {
    use super::*;

    #[test]
    fn roman_number_value_should_return_related_decimal_value_when_given_roman_number_as_a_key() {
        assert_eq!(Some(&1), ROMAN_NUMBER_VALUE.get(&'I'));
        assert_eq!(Some(&5), ROMAN_NUMBER_VALUE.get(&'V'));
    }

    #[test]
    fn get_dec_value_from_roman_number_should_return_decimal_value_when_given_single_roman_number_char(
    ) {
        assert_eq!(Some(1), get_dec_value_from_roman_number("I"));
        assert_eq!(Some(100), get_dec_value_from_roman_number("C"));
    }

    #[test]
    fn get_dec_value_from_roman_number_should_return_second_deduct_by_first_value_when_given_two_legit_roman_number_chars(
    ) {
        assert_eq!(Some(4), get_dec_value_from_roman_number("IV"));
        assert_eq!(Some(40), get_dec_value_from_roman_number("XL"));
    }

    #[test]
    fn get_dec_value_from_roman_number_should_return_sum_of_all_values_when_given_three_legit_roman_number_chars(
    ) {
        assert_eq!(Some(3), get_dec_value_from_roman_number("III"));
        assert_eq!(Some(3000), get_dec_value_from_roman_number("MMM"));
    }

    #[test]
    fn is_legit_roman_number_should_return_true_when_given_roman_number_is_legit_single_char() {
        assert!(is_legit_roman_number("X"));
    }

    #[test]
    fn is_legit_roman_number_should_return_true_when_given_roman_number_is_two_char_which_the_first_char_is_lower_than_the_second_one(
    ) {
        assert!(is_legit_roman_number("IV"));
        assert!(is_legit_roman_number("CM"));
        assert!(is_legit_roman_number("XC"));
    }

    #[test]
    fn is_legit_roman_number_should_return_true_when_given_roman_number_is_three_char_which_all_are_the_same(
    ) {
        assert!(is_legit_roman_number("III"));
        assert!(is_legit_roman_number("CCC"));
        assert!(is_legit_roman_number("XXX"));
    }

    #[test]
    fn roman_to_int_should_return_decimal_value_when_given_legit_roman_number() {
        // assert_eq!(1, Solution::roman_to_int("I".to_owned()));
        // assert_eq!(4, Solution::roman_to_int("IV".to_owned()));
        // assert_eq!(9, Solution::roman_to_int("IX".to_owned()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_owned()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_owned()));
    }
}
