/**
 * [53] Maximum Subarray
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 *  
 * Example 1:
 * 
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 * 
 * Example 2:
 * 
 * Input: nums = [1]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: nums = [5,4,-1,7,8]
 * Output: 23
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^5 <= nums[i] <= 10^5
 * 
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray/
// discuss: https://leetcode.com/problems/maximum-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sums = Vec::with_capacity(nums.len()+1);
        let (mut cur_sum, mut min_sum, mut max_sum, mut max_num) = (0, 0, 0, i32::MIN);
        let (mut min_sum_idx, mut max_sum_idx, mut max_idx) = (0, 0, 0);

        sums.push(0);
        for (idx, &num) in nums.iter().enumerate() {
            if max_num <= num {
                max_idx = idx+1;
                max_num = num;
            }

            cur_sum += num;
            if cur_sum < min_sum {
                min_sum = cur_sum;
                min_sum_idx = idx+1;
            }
            if max_sum < cur_sum {
               max_sum = cur_sum; 
               max_sum_idx = idx+1;
            }
            sums.push(cur_sum);
        }
        print!("sums: {:?}, max_sum: {}, min_sum: {}", sums, max_sum, min_sum);
        if min_sum_idx < max_sum_idx {
            return max_sum - min_sum
        } else {
            return max_num
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![-8]), -8);
        assert_eq!(Solution::max_sub_array(vec![-8, -2]), -2);
        assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}
