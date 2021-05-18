/**
 * [335] Self Crossing
 *
 * You are given an array of integers distance.
 * You start at point (0,0) on an X-Y plane and you move distance[0] meters to the north, then distance[1] meters to the west, distance[2] meters to the south, distance[3] meters to the east, and so on. In other words, after each move, your direction changes counter-clockwise.
 * Return true if your path crosses itself, and false if it does not.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross1-plane.jpg" style="width: 400px; height: 435px;" />
 * Input: distance = [2,1,1,2]
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross2-plane.jpg" style="width: 400px; height: 435px;" />
 * Input: distance = [1,2,3,4]
 * Output: false
 * 
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross3-plane.jpg" style="width: 400px; height: 435px;" />
 * Input: distance = [1,1,1,1]
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	1 <= distance.length <= 500
 * 	1 <= distance[i] <= 500
 * 
 *  
 * Follow up: Could you write a one-pass algorithm with O(1) extra space?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/self-crossing/
// discuss: https://leetcode.com/problems/self-crossing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_self_crossing(d: Vec<i32>) -> bool {
        for i in (0..d.len()) {
            if 3 <= i && d[i-2] <= d[i] && d[i-1] <= d[i-3] {
                return true;
            }

            if 4 <= i && d[i-1] == d[i-3] && d[i] + d[i-4] >= d[i-2] {
                return true;
            }

            if 5 <= i && d[i-2] > d[i-4] && d[i-4]+d[i]>=d[i-2] && d[i-3]>d[i-1] && d[i-5]+d[i-1]>=d[i-3]{
                return true;
            }

        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_335() {
        assert!(Solution::is_self_crossing(vec![2,1,1,2]));
        assert!(!Solution::is_self_crossing(vec![1,2,3,4]));
        assert!(Solution::is_self_crossing(vec![1,1,1,1]));
    }
}
