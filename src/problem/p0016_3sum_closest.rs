/**
 * [16] 3Sum Closest
 *
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 *  
 * Example 1:
 * 
 * Input: nums = [-1,2,1,-4], target = 1
 * Output: 2
 * Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 * 
 *  
 * Constraints:
 * 
 * 	3 <= nums.length <= 10^3
 * 	-10^3 <= nums[i] <= 10^3
 * 	-10^4 <= target <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-closest/
// discuss: https://leetcode.com/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut min_diff = 100000000;
        let mut nearest_sum = target;
        for (i, &num) in nums.iter().enumerate() {
            let mut low = i + 1;
            let mut high = nums.len() - 1;
            // println!("i = {}", i);
            while low < high {
                let cur_sum = nums[i] + nums[low] + nums[high];
                // println!("\tlow = {}, high = {}, sum = {}", low, high, cur_sum);

                let mut diff : i32;
                if target < cur_sum {
                   diff = cur_sum - target; 
                   high -= 1;
                } else {
                   diff = target - cur_sum;
                   low += 1;
                }

                if diff < min_diff {
                    min_diff = diff;
                    nearest_sum = cur_sum;
                }
            }

            // println!("==============================");
        }
        nearest_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1,2,1,-4], 1), 2);

    }
}
