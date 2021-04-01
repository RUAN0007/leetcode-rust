/**
 * [930] Binary Subarrays With Sum
 *
 * In an array A of 0s and 1s, how many non-empty subarrays have sum S?
 * 
 *  
 * 
 * Example 1:
 * 
 * 
 * Input: A = <span id="example-input-1-1">[1,0,1,0,1]</span>, S = <span id="example-input-1-2">2</span>
 * Output: <span id="example-output-1">4</span>
 * Explanation: 
 * The 4 subarrays are bolded below:
 * [1,0,1,0,1]
 * [1,0,1,0,1]
 * [1,0,1,0,1]
 * [1,0,1,0,1]
 * 
 * 
 *  
 * 
 * Note:
 * 
 * <ol>
 * 	A.length <= 30000
 * 	0 <= S <= A.length
 * 	A[i] is either 0 or 1.
 * </ol>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-subarrays-with-sum/
// discuss: https://leetcode.com/problems/binary-subarrays-with-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
        let mut cur_zero_count = 0;
        let mut zero_counts = vec![];
        let s = s as usize;
        for &num in &a {
            if num == 1 {
                zero_counts.push(cur_zero_count);
                cur_zero_count = 0;
            } else {
                cur_zero_count += 1;
            }
        }
        zero_counts.push(cur_zero_count);
        println!("Zero_counts: {:?}", zero_counts);
        let mut result = 0; 
        if s == 0 {
            for i in 0..zero_counts.len()- s {
                result += (zero_counts[i]) *  (zero_counts[i] + 1) / 2;
            }
        } else {
            for i in 0..zero_counts.len()- s {
                result += (zero_counts[i] + 1) *  (zero_counts[i+s] + 1);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_930() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1,0,1,0,1], 2), 4);
        assert_eq!(Solution::num_subarrays_with_sum(vec![1,0,1,0,1], 3), 1);
    }
}
