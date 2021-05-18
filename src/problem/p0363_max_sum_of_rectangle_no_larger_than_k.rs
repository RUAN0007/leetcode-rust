/**
 * [363] Max Sum of Rectangle No Larger Than K
 *
 * Given an m x n matrix matrix and an integer k, return the max sum of a rectangle in the matrix such that its sum is no larger than k.
 * It is guaranteed that there will be a rectangle with a sum no larger than k.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/18/sum-grid.jpg" style="width: 255px; height: 176px;" />
 * Input: matrix = [[1,0,1],[0,-2,3]], k = 2
 * Output: 2
 * Explanation: Because the sum of the blue rectangle [[0, 1], [-2, 3]] is 2, and 2 is the max number no larger than k (k = 2).
 * 
 * Example 2:
 * 
 * Input: matrix = [[2,2,-1]], k = 3
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 100
 * 	-100 <= matrix[i][j] <= 100
 * 	-10^5 <= k <= 10^5
 * 
 *  
 * Follow up: What if the number of rows is much larger than the number of columns?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// discuss: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_le_pos(sorted : &Vec<i32>, target : i32) -> i32 {
        let mut start : i32 = 0;
        let mut end : i32 = sorted.len() as i32 - 1;
        while start <= end {
            let mid : usize = (start + (end - start) / 2) as usize;
            if sorted[mid] <= target {
                if mid == sorted.len() - 1 || !(sorted[mid+1] <= target) {
                    return mid as i32;
                } else {
                    start = mid as i32 + 1;
                }
            } else {
                end = mid as i32 - 1;
            }
        }
        -1i32
    }

    pub fn compute_while_mergesort(mut prefix_sums : Vec<i32>, k : i32, result : &mut i32) -> Vec<i32> {
        let n : usize = prefix_sums.len();
        if n >= 2 {
            let mut sorted_right : Vec<i32> = Self::compute_while_mergesort(prefix_sums.split_off(n/2), k, result);
            let mut sorted_left : Vec<i32> = Self::compute_while_mergesort(prefix_sums, k, result);

            let mut reverse_sorted : Vec<i32> = vec![];

            for &left_num in sorted_left.iter() {
                let pos : i32 = Self::last_le_pos(&sorted_right, k + left_num);
                if pos != -1 {
                    *result = std::cmp::max(*result, sorted_right[pos as usize] - left_num);
                }
            }

            while sorted_left.len() != 0 || sorted_right.len() != 0 {
                if sorted_right.len() == 0 || sorted_left.len() != 0 && *sorted_left.last().unwrap() > *sorted_right.last().unwrap() {
                    reverse_sorted.push(sorted_left.pop().unwrap());
                } else {
                    reverse_sorted.push(sorted_right.pop().unwrap());
                }
            }
            reverse_sorted.into_iter().rev().collect()
        } else {
            prefix_sums
        }
    }

    pub fn greatest_subarray_le_target(nums : &Vec<i32>, k : i32) -> i32 {
        let mut cur_prefix_sum : i32 = 0;
        let mut prefix_sums : Vec<i32> = vec![0];

        for &num in nums.iter() {
            cur_prefix_sum += num;
            prefix_sums.push(cur_prefix_sum);
        }
        let mut result : i32 = -10000;
        Self::compute_while_mergesort(prefix_sums, k, &mut result);
        result
    }

    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_count : usize = matrix.len();
        let col_count : usize = matrix[0].len();

        let mut result : i32 = -10000;
        for left in 0..col_count {
            let mut col_sums : Vec<i32> = vec![0; row_count];
            for right in left..col_count {
                // col_sums for each row from left(inclusive) to right(inclusive)
                for i in 0..row_count {
                    col_sums[i] += matrix[i][right]
                }
                
                // may apply Kadane's Alg to find the max subarray in col_sums, and then max for the sub-matrix for each left and right. 

                // here, we find the subarray with greater sum <= k. 
                let this_result = Self::greatest_subarray_le_target(&col_sums, k);
                // println!("left={}, right={}, col_sums={:?}, this_max={}", left, right, col_sums, this_result);
                result = std::cmp::max(result, this_result);
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
    fn test_363() {
        // assert_eq!(Solution::max_sum_submatrix(vec![vec![1,0,1],vec![0,-2,3]], 2), 2);
        // assert_eq!(Solution::max_sum_submatrix(vec![vec![2,2,-1]], 3), 3);
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2,2,-1]], 0), -1);
    }
}
