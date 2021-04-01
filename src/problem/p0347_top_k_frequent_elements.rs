/**
 * [347] Top K Frequent Elements
 *
 * Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
 *  
 * Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *  
 * Constraints:
 * 
 * 	1 <= nums.legth <= 10^5
 * 	k is in the range [1, the number of unique elements in the array].
 * 	It is guaranteed that the answer is unique.
 * 
 *  
 * Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/top-k-frequent-elements/
// discuss: https://leetcode.com/problems/top-k-frequent-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        for &num in &nums {
            if let Some(c) = counts.get_mut(&num) {
                *c +=1 ;
            } else {
                counts.insert(num, 1usize);
            }
        }

        let mut sorted_count = vec![];
        for (num, count) in counts {
            sorted_count.push((count, num));
        }
        sorted_count.sort();

        let mut result = vec![];
        let mut start = 0usize;
        if (k as usize) < sorted_count.len() {
            start = sorted_count.len() - k as usize;
        }
        
        for i in start..sorted_count.len() {
            result.push(sorted_count[i].1);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_347() {
        assert_eq!(Solution::top_k_frequent(vec![1,1,1,2,2,3], 2), vec![2,1]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
