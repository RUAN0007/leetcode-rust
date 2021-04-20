/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;" />
 * Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 * Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
 * 
 * Example 2:
 * 
 * Input: height = [4,2,0,3,2,5]
 * Output: 9
 * 
 *  
 * Constraints:
 * 
 * 	n == height.length
 * 	0 <= n <= 3 * 10^4
 * 	0 <= height[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water/
// discuss: https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut left_max = vec![-1i32;n]; //exclusive of its own
        let mut cur_left_max = 0;
        for i in 0..n {
            if cur_left_max <= heights[i] {
                cur_left_max = heights[i];
            }
            left_max[i] = cur_left_max;
        }

        let mut right_max = vec![-1i32;n];
        let mut cur_right_max = 0;
        for i in (0..n).rev() {
            if cur_right_max < heights[i] {
                cur_right_max = heights[i];
            }
            right_max[i] = cur_right_max;
        }

        let mut sum : i32 = 0;
        for i in 0..n {
            let min_height = std::cmp::min(left_max[i], right_max[i]);
            if min_height > heights[i] {
                sum += min_height - heights[i];
            }
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {}
}
