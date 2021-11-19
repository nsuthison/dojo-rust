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
        todo!()
    }
}

#[cfg(test)]
pub mod solution_test {
    use super::*;

    #[test]
    fn roman_number_value_should_return_related_decimal_value_when_given_roman_number_as_a_key() {
        assert_eq!(Some(&1), ROMAN_NUMBER_VALUE.get(&'I'));
        assert_eq!(Some(&5), ROMAN_NUMBER_VALUE.get(&'V'));
    }
}
