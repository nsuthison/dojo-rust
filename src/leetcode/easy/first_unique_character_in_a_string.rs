use std::collections::HashMap;

struct Solution;

/// Question: https://leetcode.com/problems/first-unique-character-in-a-string/solution/
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let is_char_unique_map = Solution::create_is_char_unique_map(&s);

        for (idx, c) in s.chars().enumerate() {
            if is_char_unique_map[&c] {
                return idx as i32;
            }
        }

        -1
    }

    #[allow(clippy::map_entry)] // In this case map_entry is not make sense when we want to update false if entry is exist.
    fn create_is_char_unique_map(given_string: &str) -> HashMap<char, bool> {
        let mut is_char_unique_map: HashMap<char, bool> = HashMap::new();

        for c in given_string.chars() {
            if is_char_unique_map.contains_key(&c) {
                is_char_unique_map.insert(c, false);
            } else {
                is_char_unique_map.insert(c, true);
            }
        }

        is_char_unique_map
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
