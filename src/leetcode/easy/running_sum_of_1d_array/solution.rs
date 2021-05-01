/// Question: https://leetcode.com/problems/running-sum-of-1d-array/
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let acc_with_first_elem = vec![*(nums.first().unwrap_or(&0))];

        nums.into_iter()
            .skip(1)
            .fold(acc_with_first_elem, |mut acc, x| {
                acc.push(x + acc.last().unwrap_or(&0));

                acc
            })
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
    case(vec![1,2,3,4], vec![1,3,6,10]),
    case(vec![1,1,1,1,1], vec![1,2,3,4,5]),
    case(vec![3,1,2,10,1], vec![3,4,6,16,17])
    )]
    fn running_sum_should_return_vector_contain_summation_of_running_number_when_given_non_empty_vector(
        input: Vec<i32>,
        expected: Vec<i32>,
    ) {
        assert_eq!(Solution::running_sum(input), expected);
    }
}
