/**
 * [718] Maximum Length of Repeated Subarray
 *
 * Given two integer arrays A and B, return the maximum length of an subarray that appears in both arrays.
 * 
 * Example 1:
 * 
 * 
 * Input:
 * A: [1,2,3,2,1]
 * B: [3,2,1,4,7]
 * Output: 3
 * Explanation: 
 * The repeated subarray with maximum length is [3, 2, 1].
 * 
 * 
 *  
 * 
 * Note:
 * 
 * <ol>
 * 	1 <= len(A), len(B) <= 1000
 * 	0 <= A[i], B[i] < 100
 * </ol>
 * 
 *  
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-length-of-repeated-subarray/
// discuss: https://leetcode.com/problems/maximum-length-of-repeated-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut result = vec![vec![0;b.len() + 1];a.len() + 1];
        let mut cur_max = 0; 

        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i-1] == b[j-1] {
                    result[i][j] = result[i-1][j-1] + 1;
                    cur_max = std::cmp::max(cur_max, result[i][j]);
                }
            }
        }

        cur_max
    }

    pub fn find_length_standard(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut num2positions : HashMap<i32,Vec<usize>> = HashMap::new();
        for (i, &num) in b.iter().enumerate() {
            if let Some(ps) = num2positions.get_mut(&num) {
                ps.push(i);
            } else {
                num2positions.insert(num, vec![i]);
            }
        }
        let mut max_len = 0;
        for (a_start, &a_num) in a.iter().enumerate() {
            if let Some(b_positions) = num2positions.get(&a_num) {
                for &b_start in b_positions {
                    let mut cur_l = 0;
                    let mut a_i = a_start;
                    let mut b_i = b_start;
                    while a_i < a.len() && b_i < b.len() && a[a_i] == b[b_i] {
                        a_i+=1;
                        b_i+=1;
                        cur_l+=1;
                    }
                    max_len = std::cmp::max(max_len, cur_l);
                }
            }
        }

        max_len
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_718() {
        assert_eq!(Solution::find_length(vec![1,2,3,2,1], vec![3,2,1,4,7]), 3);
    }
}
