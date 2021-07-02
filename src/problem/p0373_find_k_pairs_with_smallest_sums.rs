/**
 * [373] Find K Pairs with Smallest Sums
 *
 * You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
 * Define a pair (u, v) which consists of one element from the first array and one element from the second array.
 * Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
 *  
 * Example 1:
 * 
 * Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 * Output: [[1,2],[1,4],[1,6]]
 * Explanation: The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 * 
 * Example 2:
 * 
 * Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * Output: [[1,1],[1,1]]
 * Explanation: The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 * 
 * Example 3:
 * 
 * Input: nums1 = [1,2], nums2 = [3], k = 3
 * Output: [[1,3],[2,3]]
 * Explanation: All possible pairs are returned from the sequence: [1,3],[2,3]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums1.length, nums2.length <= 10^4
 * 	-10^9 <= nums1[i], nums2[i] <= 10^9
 * 	nums1 and nums2 both are sorted in ascending order.
 * 	1 <= k <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
// discuss: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BinaryHeap;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // if nums1.len() > nums2.len() {
        //     return Self::k_smallest_pairs(nums2, nums1, k);
        // }
        let k : usize = k as usize;
        let mut last1 : usize = 0;
        let mut last2 : usize = 0;
        let mut r = vec![];

        // (-sum, pos1, pos2)
        let mut heap : BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        for i in 0..nums1.len() {
            let sum : i32 = nums1[i] + nums2[0];
            heap.push((-sum, i, 0));
        }

        while let Some(smallest) = heap.pop() {
            let num1_pos : usize = smallest.1;
            let num2_pos : usize = smallest.2;
            r.push(vec![nums1[num1_pos], nums2[num2_pos]]);
            if r.len() == k {break;}

            if num2_pos < nums2.len() - 1 {
                let nums2_next_pos : usize = num2_pos + 1;
                let sum : i32 = nums1[num1_pos] + nums2[nums2_next_pos];
                heap.push((-sum, num1_pos, nums2_next_pos));
            }
        }
        r
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_373() {
        assert_eq!(Solution::k_smallest_pairs(vec![1,7,11], vec![2,4,6], 3), vec![vec![1,2],vec![1,4],vec![1,6]]);

        assert_eq!(Solution::k_smallest_pairs(vec![1,1,2], vec![1,2,3], 2), vec![vec![1,1],vec![1,1]]);

        assert_eq!(Solution::k_smallest_pairs(vec![1,2], vec![3], 3), vec![vec![1,3],vec![2,3]]);
    }
}
