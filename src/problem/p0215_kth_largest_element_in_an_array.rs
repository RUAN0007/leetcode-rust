/**
 * [215] Kth Largest Element in an Array
 *
 * Given an integer array nums and an integer k, return the k^th largest element in the array.
 * Note that it is the k^th largest element in the sorted order, not the k^th distinct element.
 *  
 * Example 1:
 * Input: nums = [3,2,1,5,6,4], k = 2
 * Output: 5
 * Example 2:
 * Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
 * Output: 4
 *  
 * Constraints:
 * 
 * 	1 <= k <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-largest-element-in-an-array/
// discuss: https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_idx_in_sorted(nums : &[i32], target_idx : usize) -> i32 {
        let mut le_nums : Vec<i32> = vec![];
        let mut gt_nums : Vec<i32> = vec![];
        let pivot : i32 = nums[0];
        for &num in nums[1..].iter() {
            if num <= pivot {
                le_nums.push(num);
            } else {
                gt_nums.push(num);
            }
        }

        if le_nums.len() == target_idx {
            pivot
        } else if le_nums.len() > target_idx {
            Self::find_idx_in_sorted(&le_nums, target_idx) 
        } else {
            Self::find_idx_in_sorted(&gt_nums, target_idx - 1 - le_nums.len())
        }
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let target_sorted_idx : usize = nums.len() - k as usize;
        Self::find_idx_in_sorted(&nums, target_sorted_idx)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_215() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
