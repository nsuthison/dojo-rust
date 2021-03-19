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

    #[test]
    fn move_zeroes_when_given_vector_of_interger_should_move_all_zero_to_the_end_of_vector() {
        let mut nums:Vec<i32> = vec![0,1,0,3,12];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![1,3,12,0,0]);
    }
    #[test]
    fn move_zeroes_when_given_vector_of_interger_should_move_all_zero_to_the_end_of_vector_2() {
        let mut nums:Vec<i32> = vec![0,0,1];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![1,0,0]);
    }
}