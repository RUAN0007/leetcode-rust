/**
 * [300] Longest Increasing Subsequence
 *
 * Given an integer array nums, return the length of the longest strictly increasing subsequence.
 * A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].
 *  
 * Example 1:
 * 
 * Input: nums = [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 * 
 * Example 2:
 * 
 * Input: nums = [0,1,0,3,2,3]
 * Output: 4
 * 
 * Example 3:
 * 
 * Input: nums = [7,7,7,7,7,7,7]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 2500
 * 	-10^4 <= nums[i] <= 10^4
 * 
 *  
 * Follow up:
 * 
 * 	Could you come up with the O(n^2) solution?
 * 	Could you improve it to O(n log(n)) time complexity?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-subsequence/
// discuss: https://leetcode.com/problems/longest-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn binary_search(tails: &Vec<i32>, start_pos: usize, end_pos: usize, target: i32)->usize {
        // start_pos: inclusive, end_pos: exclusive
        let mid = (start_pos + end_pos) / 2;
        if target <= tails[mid] {
            Self::binary_search(tails, start_pos, mid, target)
        } else if tails[mid+1] < target {
            Self::binary_search(tails, mid+1, end_pos, target)
        } else {
            mid
        }
    }

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // i-th element the smallest tail element of lis of length i. 
        // tails is sorted. 
        let mut tails = vec![];
        tails.push(-10000);
        for num in nums {
            if *tails.last().unwrap() < num {
                tails.push(num);
            }  else {
                // binary search to locate a[i] < num <= a[i+1]
                // i can be the last such that a[i+1] does not exist. 
                let i = Self::binary_search(&tails, 0, tails.len(), num);
                tails[i+1] = num;
            }
        }

        (tails.len()-1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_300() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
