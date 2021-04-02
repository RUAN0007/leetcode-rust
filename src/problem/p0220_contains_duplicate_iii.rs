/**
 * [220] Contains Duplicate III
 *
 * Given an integer array nums and two integers k and t, return true if there are two distinct indices i and j in the array such that abs(nums[i] - nums[j]) <= t and abs(i - j) <= k.
 *  
 * Example 1:
 * Input: nums = [1,2,3,1], k = 3, t = 0
 * Output: true
 * Example 2:
 * Input: nums = [1,0,1,1], k = 1, t = 2
 * Output: true
 * Example 3:
 * Input: nums = [1,5,9,1,5,9], k = 2, t = 3
 * Output: false
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 2 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	0 <= k <= 10^4
 * 	0 <= t <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-iii/
// discuss: https://leetcode.com/problems/contains-duplicate-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::{collections::HashMap, hash::Hash};
impl Solution {

    fn bucket_idx(num: i64, t: i64) -> i64 {
        let mut bkt_idx = num / (t + 1) as i64;
        if num < 0 {bkt_idx -= 1;}
        bkt_idx
    }

    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i64, t: i64) -> bool {
        let mut buckets = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            println!("i = {}, num = {}", i, num);
            let num = num as i64;
            let bkt_idx = Self::bucket_idx(num, t);
            println!("\t bkt_idx = {}", bkt_idx);
            if let Some(_) = buckets.get(&(bkt_idx)) {
                return true;
            }  
            
            if let Some(&pred_num) = buckets.get(&(bkt_idx-1)) {
                println!("\t pred_num = {}", pred_num);
                if ((pred_num - num) as i64).abs() <= t {
                    return true;
                }
            } 
            
            if let Some(sub_num) = buckets.get(&(bkt_idx+1)) {
                println!("\t sub_num = {}", sub_num);
                if ((sub_num - num) as i64).abs() <= t {
                    return true;
                }
            }
            // remove (i-k)-th element from the bucket
            let k = k as usize;
            buckets.insert(bkt_idx, num);
            if k <= i {
                buckets.remove(&Self::bucket_idx(nums[i-k] as i64, t));
            }
            println!("\tbuckets = {:?}", buckets);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_220() {
        // assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,5,9,1,5,9], 2, 3), false);
        // assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,2,3,1], 3, 0), true);
        // assert_eq!(
        //     Solution::contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
        //     false
        // );
        // assert_eq!(Solution::contains_nearby_almost_duplicate(vec![-3,3,6], 2, 3), true);
        // assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,2], 0, 1), false);
        assert_eq!(Solution::contains_nearby_almost_duplicate(vec![3,6, 0, 4], 2, 2), true);
    }
}
