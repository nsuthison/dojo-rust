struct Solution {}

// Question: https://leetcode.com/problems/move-zeroes/
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;
        let mut swap_count = 0;

        while idx < nums.len() {
            if nums[idx] == 0 {
                nums.remove(idx);
                nums.append(&mut vec![0]);

                swap_count += 1;

                if swap_count > nums.len() {
                    break;
                }
            } else {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
        case(&mut vec![0,1,0,3,12], &mut vec![1,3,12,0,0]),
        case(&mut vec![0, 0, 1], &mut vec![1, 0, 0]),
    )]
    fn move_zeroes_when_given_vector_of_interger_should_move_all_zero_to_the_end_of_vector(
        input: &mut Vec<i32>,
        expected: &mut Vec<i32>,
    ) {
        Solution::move_zeroes(input);

        assert_eq!(input, expected);
    }
}
