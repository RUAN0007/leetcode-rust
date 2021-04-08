/**
 * [239] Sliding Window Maximum
 *
 * You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.
 * Return the max sliding window.
 *  
 * Example 1:
 * 
 * Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation: 
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 * 
 * Example 2:
 * 
 * Input: nums = [1], k = 1
 * Output: [1]
 * 
 * Example 3:
 * 
 * Input: nums = [1,-1], k = 1
 * Output: [1,-1]
 * 
 * Example 4:
 * 
 * Input: nums = [9,11], k = 2
 * Output: [11]
 * 
 * Example 5:
 * 
 * Input: nums = [4,-2], k = 2
 * Output: [4]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 	1 <= k <= nums.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-window-maximum/
// discuss: https://leetcode.com/problems/sliding-window-maximum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::BTreeMap;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut num_count = BTreeMap::new();
        let mut result = vec![];
        for i in 0..k {
            let num = nums[i];
            if let Some(count) = num_count.get_mut(&num) {
                *count += 1;
            } else {
                num_count.insert(num, 1i32);
            }
        }
        result.push(*num_count.iter().next_back().unwrap().0);
        for i in k..nums.len() {
            let num_to_decre = nums[i-k];
            *num_count.get_mut(&num_to_decre).unwrap()-=1;
            if num_count[&num_to_decre] == 0 {
                num_count.remove(&num_to_decre);
            }


            let num_to_incre = nums[i];

            if let Some(count) = num_count.get_mut(&num_to_incre) {
                *count += 1;
            } else {
                num_count.insert(num_to_incre, 1i32);
            }

            result.push(*num_count.iter().next_back().unwrap().0);

        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_239() {
        assert_eq!(
            Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
            vec![10, 10, 9, 2]
        );
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
            vec![3, 3, 2, 5]
        );
        assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
