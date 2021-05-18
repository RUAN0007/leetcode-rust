/**
 * [330] Patching Array
 *
 * Given a sorted integer array nums and an integer n, add/patch elements to the array such that any number in the range [1, n] inclusive can be formed by the sum of some elements in the array.
 * Return the minimum number of patches required.
 *  
 * Example 1:
 * 
 * Input: nums = [1,3], n = 6
 * Output: 1
 * Explanation:
 * Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
 * Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
 * Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
 * So we only need 1 patch.
 * 
 * Example 2:
 * 
 * Input: nums = [1,5,10], n = 20
 * Output: 2
 * Explanation: The two patches can be [2, 4].
 * 
 * Example 3:
 * 
 * Input: nums = [1,2,2], n = 5
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 10^4
 * 	nums is sorted in ascending order.
 * 	1 <= n <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/patching-array/
// discuss: https://leetcode.com/problems/patching-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut miss_count : i32 = 0;
        let mut next_miss : i64 = 1;
        let n = n as i64;
        let mut i : usize = 0;
        while next_miss <= n {
            if i < nums.len() && nums[i] as i64 <= next_miss {
                next_miss += nums[i] as i64;
                i+=1;
            } else {
                miss_count += 1;
                next_miss *= 2;
            }
        }
        miss_count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_330() {
        assert_eq!(Solution::min_patches(vec![1,3], 6), 1);
        assert_eq!(Solution::min_patches(vec![1,5,10], 20), 2);
    }
}
