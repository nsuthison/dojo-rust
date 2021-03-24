struct Solution;

/// Question: https://leetcode.com/problems/richest-customer-wealth/
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|account| account.iter().sum()).fold(
            0,
            |a, b| {
                if a > b {
                    a
                } else {
                    b
                }
            },
        )
    }
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
    case(vec![vec![1,2,3],vec![3,2,1]], 6),
    case(vec![vec![1,5],vec![7,3],vec![3,5]], 10),
    case(vec![vec![2,8,7],vec![7,1,3],vec![1,9,5]], 17),
    )]
    fn maximum_wealth_should_return_the_maximum_wealth_when_given_vector_of_customer_accounts(
        input: Vec<Vec<i32>>,
        expected: i32,
    ) {
        let result = Solution::maximum_wealth(input);

        assert_eq!(result, expected);
    }
}
