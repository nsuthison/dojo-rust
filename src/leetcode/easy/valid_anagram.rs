use std::collections::HashMap;

struct Solution;

/// Question: https://leetcode.com/problems/valid-anagram/solution/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        create_number_of_letter_map(&s) == create_number_of_letter_map(&t)
    }
}

fn create_number_of_letter_map(input: &str) -> HashMap<char, i32> {
    let mut number_of_letter: HashMap<char, i32> = HashMap::new();

    for letter in input.chars() {
        *number_of_letter.entry(letter).or_insert(0) += 1;
    }

    number_of_letter
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        first_string,
        second_string,
        case("anagram", "nagaram"),
        case("abba", "baab")
    )]
    fn is_anagram_should_return_true_when_both_of_the_string_are_anagram_to_each_others(
        first_string: &str,
        second_string: &str,
    ) {
        assert_eq!(
            Solution::is_anagram(String::from(first_string), String::from(second_string)),
            true
        );
    }
}
