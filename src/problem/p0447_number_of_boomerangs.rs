/**
 * [447] Number of Boomerangs
 *
 * You are given n points in the plane that are all distinct, where points[i] = [xi, yi]. A boomerang is a tuple of points (i, j, k) such that the distance between i and j equals the distance between i and k (the order of the tuple matters).
 * Return the number of boomerangs.
 *  
 * Example 1:
 * 
 * Input: points = [[0,0],[1,0],[2,0]]
 * Output: 2
 * Explanation: The two boomerangs are [[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]].
 * 
 * Example 2:
 * 
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 2
 * 
 * Example 3:
 * 
 * Input: points = [[1,1]]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	n == points.length
 * 	1 <= n <= 500
 * 	points[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the points are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-boomerangs/
// discuss: https://leetcode.com/problems/number-of-boomerangs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {

    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        
        let n = points.len();
        let mut result = 0;
        for i in 0..n {
            let mut distance_idxs = HashMap::new();
            for j in 0..n {
                if i == j {continue;}
                let x_diff = std::cmp::max(points[i][0], points[j][0]) - std::cmp::min(points[i][0], points[j][0]);
                let y_diff = std::cmp::max(points[i][1], points[j][1]) - std::cmp::min(points[i][1], points[j][1]);    
                let distance = x_diff * x_diff + y_diff * y_diff;
                if let Some(idx) = distance_idxs.get_mut(&distance) {
                    *idx += 1;
                } else {
                    distance_idxs.insert(distance, 1 as i32);
                }
            }
            for (_, count) in distance_idxs {
                result += count * (count - 1);
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
    fn test_447() {
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![0,0], vec![1,0], vec![2,0]]),
            2
        )
    }
}
