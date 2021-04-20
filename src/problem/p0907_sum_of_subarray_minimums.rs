/**
 * [907] Sum of Subarray Minimums
 *
 * Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 * 
 * Input: arr = [3,1,2,4]
 * Output: 17
 * Explanation: 
 * Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4]. 
 * Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
 * Sum is 17.
 * 
 * Example 2:
 * 
 * Input: arr = [11,81,94,43,3]
 * Output: 444
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 3 * 10^4
 * 	1 <= arr[i] <= 3 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subarray-minimums/
// discuss: https://leetcode.com/problems/sum-of-subarray-minimums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
#[derive(Debug)]
struct Pair(i32,  // num 
            i32,  // count
            i32   // sum of num*count for each pair in the stack, including the current one. 
        );
impl Solution {

    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let modular : i32 = 1000000000 + 7;
        let mut stack = vec![];
        let mut left_nearest_lt_idx = vec![0i32; n];
        for i in 0..n {
            while let Some(&last_idx) = stack.last() {
                if !(arr[last_idx] < arr[i]) {
                    stack.pop();
                } else {
                    break;
                }
            } 
            
            if let Some(&last_idx) = stack.last() {
                left_nearest_lt_idx[i] = last_idx as i32;
            } else {
                left_nearest_lt_idx[i] = -1i32;
            }
            stack.push(i);
        }

        stack.clear();
        let mut right_nearest_le_idx = vec![0i32; n];
        for i in (0..n).rev() {
            while let Some(&last_idx) = stack.last() {
                if !(arr[last_idx] <= arr[i])  {
                    stack.pop();
                } else {
                    break;
                }
            } 
            
            if let Some(&last_idx) = stack.last() {
                right_nearest_le_idx[i] = last_idx as i32;
            } else {
                right_nearest_le_idx[i] = n as i32;
            }
            stack.push(i);
        }

        let mut sum = 0i32;
        for i in (0..n) {
            sum = (sum + arr[i] * (i as i32- left_nearest_lt_idx[i]) * (right_nearest_le_idx[i] - i as i32)) % modular;
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_907() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![3, 1, 2, 4]),
           17 
        );
    }
}
