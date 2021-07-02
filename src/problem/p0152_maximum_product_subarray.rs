/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find a contiguous non-empty subarray within the array that has the largest product, and return the product.
 * It is guaranteed that the answer will fit in a 32-bit integer.
 * A subarray is a contiguous subsequence of the array.
 *  
 * Example 1:
 * 
 * Input: nums = [2,3,-2,4]
 * Output: 6
 * Explanation: [2,3] has the largest product 6.
 * 
 * Example 2:
 * 
 * Input: nums = [-2,0,-1]
 * Output: 0
 * Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 2 * 10^4
 * 	-10 <= nums[i] <= 10
 * 	The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-subarray/
// discuss: https://leetcode.com/problems/maximum-product-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut pre_max_pos : Option<i32> = None;
        let mut pre_max_neg : Option<i32> = None;
        let mut max_all : i32 = -20;
        for &num in nums.iter() {
            if num == 0 {
                pre_max_pos = None;
                pre_max_neg = None;
                max_all = std::cmp::max(max_all, 0);
            } else if num > 0 {
                if let Some(pre_max_pos_val) = pre_max_pos {
                    pre_max_pos = Some(pre_max_pos_val * num);
                } else {
                    pre_max_pos = Some(num);
                }

                if let Some(pre_max_neg_val) = pre_max_neg {
                    pre_max_neg = Some(pre_max_neg_val * num);
                }
            } else {
                let last_pre_max_pos = pre_max_pos;
                let last_pre_max_neg = pre_max_neg;

                if let Some(last_pre_max_neg_val) = last_pre_max_neg {
                    pre_max_pos = Some(last_pre_max_neg_val * num);
                } else {
                    pre_max_pos = None;
                }

                if let Some(last_pre_max_pos_val) = last_pre_max_pos {
                    pre_max_neg = Some(last_pre_max_pos_val * num);
                } else {
                    pre_max_neg = Some(num);
                }
            }

            if let Some(pre_max_neg_val) = pre_max_neg {
                max_all = std::cmp::max(max_all, pre_max_neg_val);
            }

            if let Some(pre_max_pos_val) = pre_max_pos {
                max_all = std::cmp::max(max_all, pre_max_pos_val);
            }
        }
        max_all 
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_152() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-4, -3, -2]), 12);
        assert_eq!(Solution::max_product(vec![-2]), -2);
    }
}
