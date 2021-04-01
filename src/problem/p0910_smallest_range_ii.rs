/**
 * [910] Smallest Range II
 *
 * Given an array A of integers, for each integer A[i] we need to choose either x = -K or x = K, and add x to A[i] (only once).
 * 
 * After this process, we have some array B.
 * 
 * Return the smallest possible difference between the maximum value of B and the minimum value of B.
 * 
 *  
 * 
 * <ol>
 * </ol>
 * 
 * <div>
 * Example 1:
 * 
 * 
 * Input: A = <span id="example-input-1-1">[1]</span>, K = <span id="example-input-1-2">0</span>
 * Output: <span id="example-output-1">0</span>
 * <span>Explanation: B = [1]</span>
 * 
 * 
 * <div>
 * Example 2:
 * 
 * 
 * Input: A = <span id="example-input-2-1">[0,10]</span>, K = <span id="example-input-2-2">2</span>
 * Output: <span id="example-output-2">6
 * </span><span>Explanation: B = [2,8]</span>
 * 
 * 
 * <div>
 * Example 3:
 * 
 * 
 * Input: A = <span id="example-input-3-1">[1,3,6]</span>, K = <span id="example-input-3-2">3</span>
 * Output: <span id="example-output-3">3</span>
 * <span>Explanation: B = [4,6,3]</span>
 * 
 * 
 *  
 * 
 * Note:
 * 
 * <ol>
 * 	1 <= A.length <= 10000
 * 	0 <= A[i] <= 10000
 * 	0 <= K <= 10000
 * </ol>
 * </div>
 * </div>
 * </div>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-range-ii/
// discuss: https://leetcode.com/problems/smallest-range-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Refer to the discussion here. 
    pub fn smallest_range_ii(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort();

        // for the case when all elements share the same operation. 
        let mut diff = a.last().unwrap() - a[0];

        // Else: the result array must be of format:
        // a[0] + k, a[1] + k, a[p] + k, a[p+1] -k ... a[n-1]-k
        // Given p, the min of the array is min(a[0]+k, a[p+1]-k)
        //  and the max is max(a[p]+k, a[n-1]-k). 
        // Iterate p to minimize the gap between max - min. 
        let n = a.len();
        for p in 0..n-1 {
            let min = std::cmp::min(a[0]+k, a[p+1]-k);
            let max = std::cmp::max(a[p]+k, a[n-1]-k);
            diff = std::cmp::min(diff, max - min);
        }
        diff
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_910() {
    }
}
