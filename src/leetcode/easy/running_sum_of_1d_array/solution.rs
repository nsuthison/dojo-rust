/// Question: https://leetcode.com/problems/running-sum-of-1d-array/
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().fold(vec![], |mut acc, x| {
            if acc.is_empty() {
                acc.push(x);
            } else {
                acc.push(x + acc.last().unwrap_or(&0));
            }

            acc
        })
    }
}

struct Solution;
