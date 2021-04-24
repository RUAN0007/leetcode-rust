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
    pub fn find_last_list_index_with_smaller_end(lises : &Vec<Vec<i32>>, target : i32) -> i32 {
        let mut low = 0i32;
        let mut high = lises.len() as i32 - 1;
        let n = lises.len() as i32;
        while low <= high {
            let mid = ((low + high) / 2) as usize;
            let mid_num = *lises[mid].last().unwrap();
            // println!("low={}, high={}, mid={}, mid_num={}", low, high, mid, mid_num);
            if mid_num < target {
                if mid as i32 == (n-1) || !(*lises[mid+1].last().unwrap() < target) {
                    return mid as i32;
                } else {
                    low = mid as i32 + 1;
                }
            } else {
                high = mid as i32 - 1;
            }
        }
        -1 // not found. 
    }

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // lises[i] maintains the longest increasing subsequent of length i, known so far. 
        let mut lises = vec![];
        for &num in nums.iter() {
            let l = Self::find_last_list_index_with_smaller_end(&lises, num);
            let mut new_list : Vec<i32> = vec![];
            if l != -1 {
                new_list = lises[l as usize].clone();
            }
            new_list.push(num);
            // println!("num={}, l={}, new_list={:?}", num, l, new_list);
            // println!("lises={:?}", lises);
            // println!("=========================================");
            let insert_idx = (l + 1) as usize;
            if insert_idx < lises.len() {
                lises[insert_idx] = new_list;
            } else if insert_idx == lises.len() {
                lises.push(new_list);
            } else {
                panic!("Impossible...");
            }
        }
        // println!("lis={:?}", *lises.last().unwrap());
        lises.last().unwrap().len() as i32
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
