/// Question: https://leetcode.com/problems/reverse-string/
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        for (front_idx, back_idx) in ((s.len() / 2)..(s.len())).rev().enumerate() {
            s.swap(front_idx, back_idx);
        }
    }
}

struct Solution;

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    input,
    expected,
    case(vec!['h','e','l','l','o'], vec!['o','l','l','e','h']),
    case(vec!['H','a','n','n','a','h'], vec!['h','a','n','n','a','H']),
    )]
    fn reverse_string_should_return_vector_contain_reverse_char_from_the_original_one_when_given_non_empty_vector(
        input: Vec<char>,
        expected: Vec<char>,
    ) {
        let mut input_mut = input;

        Solution::reverse_string(&mut input_mut);

        assert_eq!(input_mut, expected);
    }
}
