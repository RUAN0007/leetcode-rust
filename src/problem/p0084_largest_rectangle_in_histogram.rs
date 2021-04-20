/**
 * [84] Largest Rectangle in Histogram
 *
 * Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg" style="width: 522px; height: 242px;" />
 * Input: heights = [2,1,5,6,2,3]
 * Output: 10
 * Explanation: The above is a histogram where width of each bar is 1.
 * The largest rectangle is shown in the red area, which has an area = 10 units.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg" style="width: 202px; height: 362px;" />
 * Input: heights = [2,4]
 * Output: 4
 * 
 *  
 * Constraints:
 * 
 * 	1 <= heights.length <= 10^5
 * 	0 <= heights[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-rectangle-in-histogram/
// discuss: https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// use std::collections::VecDeque;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut left_rightmost_smaller_idx = vec![-1i32;n];
        let mut stack : Vec<usize> = vec![];
        for (i, &height) in heights.iter().enumerate() {
            while let Some(&last_idx) = stack.last() {
                if !(heights[last_idx] < height) {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&last_idx) = stack.last() {
                left_rightmost_smaller_idx[i] = last_idx as i32;
            } else {
                left_rightmost_smaller_idx[i] = -1;
            }
            stack.push(i);
        }

        let mut stack : Vec<usize> = vec![];
        let mut right_leftmost_smaller_idx = vec![n as i32;n];
        for (i, &height) in heights.iter().enumerate().rev() {
            while let Some(&last_idx) = stack.last() {
                if !(heights[last_idx] < height) {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&last_idx) = stack.last() {
                right_leftmost_smaller_idx[i] = last_idx as i32;
            } else {
                right_leftmost_smaller_idx[i] = n as i32;
            }
            stack.push(i);
        }

        let mut max_area : i32 = 0;
        // println!("heights={:?}", heights);
        // println!("left_rightmost_smaller_idx={:?}", left_rightmost_smaller_idx);
        // println!("right_leftmost_smaller_idx={:?}", right_leftmost_smaller_idx);
        for i in 0..n {
            max_area = std::cmp::max(max_area, heights[i] * (right_leftmost_smaller_idx[i] - left_rightmost_smaller_idx[i]-1));
        }
        max_area
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(
            Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1]),
            8
        );
        assert_eq!(Solution::largest_rectangle_area(vec![2, 2]), 4);
        assert_eq!(
            Solution::largest_rectangle_area(vec![1, 2, 8, 8, 2, 2, 1]),
            16
        );
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
        assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 2]), 5);
    }
}
