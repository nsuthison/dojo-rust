struct Solution;

/// Question: https://leetcode.com/problems/kth-largest-element-in-an-array/
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut sort_nums = nums;

        sort_nums.sort_unstable();
        sort_nums.reverse();

        let result: &i32 = sort_nums.get(k as usize - 1).unwrap();

        *result
    }
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    nums,
    kth,
    expected,
    case(vec![3,2,1,5,6,4], 2, 5),
    case(vec![3,2,3,1,2,4,5,5,6], 4, 4)
    )]
    fn find_kth_largest_should_return_largest_element_in_given_sort_order_when_called(
        nums: Vec<i32>,
        kth: i32,
        expected: i32,
    ) {
        assert_eq!(expected, Solution::find_kth_largest(nums, kth));
    }
}
