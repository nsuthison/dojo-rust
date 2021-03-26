struct Solution;

/// Question: https://leetcode.com/problems/valid-palindrome/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = get_lower_case_alpha_numeric(&s);
        s == s.chars().rev().collect::<String>()
    }
}

pub fn get_lower_case_alpha_numeric(s: &str) -> String {
    s.chars().filter(|x| x.is_alphanumeric()).collect::<String>().to_lowercase()
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(input,
    case("A man, a plan, a canal: Panama"),
    case("abba"),
    )]
    fn is_palindrome_should_return_true_when_given_str_is_palindrome(
        input: &str,
    ) {
        assert_eq!(Solution::is_palindrome(String::from(input)), true);
    }

    #[rstest(input,
    case("race a car"),
    case("abbcb"),
    )]
    fn is_palindrome_should_return_false_when_given_str_is_not_palindrome(
        input: &str,
    ) {
        assert_eq!(Solution::is_palindrome(String::from(input)), false);
    }
}