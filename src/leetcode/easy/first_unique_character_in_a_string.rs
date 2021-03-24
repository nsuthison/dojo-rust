use std::collections::HashMap;

struct Solution;

/// Question: https://leetcode.com/problems/first-unique-character-in-a-string/solution/
#[allow(clippy::map_entry)] // In this case map_entry is not make sense when we want to update false if entry is exist.
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut alphabet_is_unique: HashMap<char, bool> = HashMap::new();

        for (_, c) in s.chars().enumerate() {
            if alphabet_is_unique.contains_key(&c) {
                alphabet_is_unique.insert(c, false);
            } else {
                alphabet_is_unique.insert(c, true);
            }
        }

        for (idx, c) in s.chars().enumerate() {
            if alphabet_is_unique[&c] {
                return idx as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected, case("leetcode", 0), case("loveleetcode", 2))]
    fn first_uniq_char_should_return_first_idx_of_unique_char_when_given_string_with_unique_char(
        input: &str,
        expected: i32,
    ) {
        let result = Solution::first_uniq_char(String::from(input));

        assert_eq!(result, expected);
    }
}
