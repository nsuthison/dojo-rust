struct Solution {}

// Question: https://leetcode.com/problems/move-zeroes/
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;
        while idx < nums.len() {
            if nums[idx] == 0 {
                nums.remove(idx);
                nums.append(&mut vec![0]);
                if is_the_rest_only_zero(&nums, idx) {
                    break;
                }
            } else {
                idx = idx + 1;
            }
        }
    }
}

fn is_the_rest_only_zero(nums: &Vec<i32>, current_idx: usize) -> bool {
    for idx in current_idx..(nums.len() - 1) {
        if nums[idx] != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected, 
        case(&mut vec![0,1,0,3,12], &mut vec![1,3,12,0,0]),
        case(&mut vec![0, 0, 1], &mut vec![1, 0, 0]),
    )]
    fn move_zeroes_when_given_vector_of_interger_should_move_all_zero_to_the_end_of_vector(input: &mut Vec<i32>, expected: &mut Vec<i32>,) {
        Solution::move_zeroes(input);

        assert_eq!(input, expected);
    }
}
