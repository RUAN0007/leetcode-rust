/**
 * [713] Subarray Product Less Than K
 *
 * Your are given an array of positive integers nums.
 * Count and print the number of (contiguous) subarrays where the product of all the elements in the subarray is less than k.
 * 
 * Example 1:<br />
 * 
 * Input: nums = [10, 5, 2, 6], k = 100
 * Output: 8
 * Explanation: The 8 subarrays that have product less than 100 are: [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6].
 * Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
 * 
 * 
 * 
 * Note:
 * 0 < nums.length <= 50000.
 * 0 < nums[i] < 1000.
 * 0 <= k < 10^6.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-product-less-than-k/
// discuss: https://leetcode.com/problems/subarray-product-less-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut low = 0usize;
        let mut high = 0usize;
        let mut cur_product = 1;
        let mut count = 0;
        while high < nums.len() {
            cur_product *= nums[high];
            while k <= cur_product && low <= high {
                cur_product /= nums[low];
                low+=1;
            }
            count+=(high-low+1);
            high+=1;
        }
        count as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_713() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![10,5,2,6], 100), 8);
    }
}
