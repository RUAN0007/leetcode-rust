/**
 * [480] Sliding Window Median
 *
 * Median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value. So the median is the mean of the two middle value.
 * Examples:
 * [2,3,4] , the median is 3
 * [2,3], the median is (2 + 3) / 2 = 2.5
 * Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position. Your job is to output the median array for each window in the original array.
 * For example,<br />
 * Given nums = [1,3,-1,-3,5,3,6,7], and k = 3.
 * 
 * Window position                Median
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       1
 *  1 [3  -1  -3] 5  3  6  7       -1
 *  1  3 [-1  -3  5] 3  6  7       -1
 *  1  3  -1 [-3  5  3] 6  7       3
 *  1  3  -1  -3 [5  3  6] 7       5
 *  1  3  -1  -3  5 [3  6  7]      6
 * 
 * Therefore, return the median sliding window as [1,-1,-1,3,5,6].
 * Note: <br />
 * You may assume k is always valid, ie: k is always smaller than input array's size for non-empty array.<br />
 * Answers within 10^-5 of the actual value will be accepted as correct.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-window-median/
// discuss: https://leetcode.com/problems/sliding-window-median/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::BTreeMap;
impl Solution {
    pub fn median(left_half: &BTreeMap<i32, usize>, right_half: &BTreeMap<i32, usize>, k : usize) -> f64 {

        // println!("left_half={:?}, right_half={:?}", left_half, right_half);
        if k % 2 == 1 {
            *right_half.iter().next().unwrap().0 as f64
        } else {
            let right_min = *right_half.iter().next().unwrap().0 as f64;
            let left_max = *left_half.iter().next_back().unwrap().0 as f64;
            (right_min + left_max) / 2f64
        }
    }

    pub fn decrement_for_key(half : &mut BTreeMap<i32, usize>, key : i32) {
        if half.contains_key(&key) {
            *half.get_mut(&key).unwrap() -=1;
            if half[&key] == 0 {
                half.remove(&key);
            }
        }
    }

    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut first_k_sorted = nums.split_at(k).0.to_vec();
        first_k_sorted.sort();

        let mut left_half = BTreeMap::new();
        let mut right_half = BTreeMap::new();
        let mid_num = first_k_sorted[k / 2];
        // println!("first_k_sorted={:?}", first_k_sorted);

        for &num in &first_k_sorted[0..k/2] {
            *(left_half.entry(num).or_insert(0))+=1;
        }

        for &num in &first_k_sorted[k/2..] {
            *(right_half.entry(num).or_insert(0))+=1;
        }

        let mut result = vec![];
        result.push(Self::median(&left_half, &right_half, k));

        for i in k..nums.len() {
            let num_to_add = nums[i];
            let num_to_remove = nums[i-k];


            if left_half.contains_key(&num_to_remove) {
                Self::decrement_for_key(&mut left_half, num_to_remove);

                *(right_half.entry(num_to_add).or_insert(0))+=1;
                let right_min : i32 = *right_half.iter().next().unwrap().0;
                Self::decrement_for_key(&mut right_half, right_min);

                *(left_half.entry(right_min).or_insert(0))+=1;
            } else if right_half.contains_key(&num_to_remove) {
                Self::decrement_for_key(&mut right_half, num_to_remove);

                *(left_half.entry(num_to_add).or_insert(0))+=1;
                let left_max : i32 = *left_half.iter().next_back().unwrap().0;
                Self::decrement_for_key(&mut left_half, left_max);

                *(right_half.entry(left_max).or_insert(0))+=1;
            } else {
                panic!("Fail to find num={} to remove at left_half={:?}, and right_half={:?}", num_to_remove, left_half, right_half);
            }

            result.push(Self::median(&left_half, &right_half, k));

        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_480() {
        assert_eq!(Solution::median_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![1.0,-1.0,-1.0,3.0,5.0,6.0]);

        assert_eq!(Solution::median_sliding_window(vec![1,4,2,3], 4),vec![2.5] );
        assert_eq!(Solution::median_sliding_window(vec![2147483647,2147483647]     ,2),vec![2147483647.0]);

        assert_eq!(Solution::median_sliding_window(vec![-2147483648,-2147483648,2147483647,-2147483648,-2147483648,-2147483648,2147483647,2147483647,2147483647,2147483647,-2147483648,2147483647,-2147483648]
            ,1), vec![-2147483648,-2147483648,2147483647,-2147483648,-2147483648,-2147483648,2147483647,2147483647,2147483647,2147483647,-2147483648,2147483647,-2147483648].iter().map(|x|{*x as f64}).collect::<Vec<f64>>());

    }
}
