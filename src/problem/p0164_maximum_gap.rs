/**
 * [164] Maximum Gap
 *
 * Given an integer array nums, return the maximum difference between two successive elements in its sorted form. If the array contains less than two elements, return 0.
 *  
 * Example 1:
 * 
 * Input: nums = [3,6,9,1]
 * Output: 3
 * Explanation: The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
 * 
 * Example 2:
 * 
 * Input: nums = [10]
 * Output: 0
 * Explanation: The array contains less than 2 elements, therefore return 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 10^9
 * 
 *  
 * Follow up: Could you solve it in linear time/space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-gap/
// discuss: https://leetcode.com/problems/maximum-gap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{return 0};
        let max_num = *nums.iter().max().unwrap();
        let min_num = *nums.iter().min().unwrap();
        if nums.len() == 2{return max_num - min_num};

        // prepare for n-1 buckets with at most n - 2 values, excluding min and max. 
        // As there exists at least one empty bucket, the max gap must appear for the max of a bucket and the min of a next bucket. This is because the diff between nums in the same bucket is within the bucket gap. 
        let n = nums.len() as i32;
        // ceil division
        let bucket_gap = if (max_num - min_num) % (n-2) == 0 {(max_num - min_num) / (n-2)} else {(max_num - min_num) / (n-2)+1};

        let mut bucket_min = vec![max_num;(n-2) as usize];
        let mut bucket_max = vec![min_num;(n-2) as usize];
        for num in nums {
            if num == min_num || num == max_num {continue}
            let bucket_idx = (num - min_num) / bucket_gap;
            let bucket_idx = bucket_idx as usize;
            bucket_min[bucket_idx] = std::cmp::min(bucket_min[bucket_idx], num);
            bucket_max[bucket_idx] = std::cmp::max(bucket_max[bucket_idx], num);
        }

        let mut prev_max = min_num;
        let mut max_gap = 0;
        for i in 0..n-2 {
            // empty bucket
            let i = i as usize;
            if bucket_min[i] == max_num && bucket_max[i] == min_num {continue}
            max_gap = std::cmp::max(max_gap, bucket_min[i]-prev_max);
            prev_max = bucket_max[i];
        }
        std::cmp::max(max_gap, max_num - prev_max)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_164() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    }
}
