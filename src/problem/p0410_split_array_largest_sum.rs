/**
 * [410] Split Array Largest Sum
 *
 * Given an array nums which consists of non-negative integers and an integer m, you can split the array into m non-empty continuous subarrays.
 * Write an algorithm to minimize the largest sum among these m subarrays.
 *  
 * Example 1:
 * 
 * Input: nums = [7,2,5,10,8], m = 2
 * Output: 18
 * Explanation:
 * There are four ways to split nums into two subarrays.
 * The best way is to split it into [7,2,5] and [10,8],
 * where the largest sum among the two subarrays is only 18.
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,3,4,5], m = 2
 * Output: 9
 * 
 * Example 3:
 * 
 * Input: nums = [1,4,4], m = 3
 * Output: 4
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] <= 10^6
 * 	1 <= m <= min(50, nums.length)
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-largest-sum/
// discuss: https://leetcode.com/problems/split-array-largest-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid(nums: &Vec<i32>, m: i32, sub_sum: i32) -> bool {
        let mut cur_sum = 0;
        let mut break_count = 0;
        for &num in nums {
            cur_sum += num;
            if sub_sum < cur_sum {
                cur_sum = num;
                break_count += 1;
                if m-1 < break_count {return false}
            }
        }
        true
    }

    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let max = *nums.iter().max().unwrap();
        let sum = nums.iter().sum();

        let mut low = max;
        let mut high = sum;
        while low != high {
            let mid = (low+high) / 2;
            if Self::valid(&nums, m, mid) {
                high = mid ;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_410() {
        assert_eq!(Solution::split_array(vec![7,2,5,10,8], 2), 18);
    }
}
