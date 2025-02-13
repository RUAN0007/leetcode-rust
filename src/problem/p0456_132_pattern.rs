/**
 * [456] 132 Pattern
 *
 * Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
 * Return true if there is a 132 pattern in nums, otherwise, return false.
 * Follow up: The O(n^2) is trivial, could you come up with the O(n logn) or the O(n) solution?
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,4]
 * Output: false
 * Explanation: There is no 132 pattern in the sequence.
 * 
 * Example 2:
 * 
 * Input: nums = [3,1,4,2]
 * Output: true
 * Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
 * 
 * Example 3:
 * 
 * Input: nums = [-1,3,2,0]
 * Output: true
 * Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
 * 
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/132-pattern/
// discuss: https://leetcode.com/problems/132-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find132pattern(mut nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        nums.reverse();
        let mut s3 = -10000000;
        for e in nums {
            if e < s3 { return true; }
            while let Some(&last) = stack.last() {
                if last < e {
                    s3 = last;
                    stack.pop();
                } else {
                    break
                }
            }
            stack.push(e);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_456() {
    }
}
