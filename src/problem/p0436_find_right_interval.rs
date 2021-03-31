/**
 * [436] Find Right Interval
 *
 * You are given an array of intervals, where intervals[i] = [starti, endi] and each starti is unique.
 * The right interval for an interval i is an interval j such that startj >= endi and startj is minimized.
 * Return an array of right interval indices for each interval i. If no right interval exists for interval i, then put -1 at index i.
 *  
 * Example 1:
 * 
 * Input: intervals = [[1,2]]
 * Output: [-1]
 * Explanation: There is only one interval in the collection, so it outputs -1.
 * 
 * Example 2:
 * 
 * Input: intervals = [[3,4],[2,3],[1,2]]
 * Output: [-1,0,1]
 * Explanation: There is no right interval for [3,4].
 * The right interval for [2,3] is [3,4] since start0 = 3 is the smallest start that is >= end1 = 3.
 * The right interval for [1,2] is [2,3] since start1 = 2 is the smallest start that is >= end2 = 2.
 * 
 * Example 3:
 * 
 * Input: intervals = [[1,4],[2,3],[3,4]]
 * Output: [-1,2,-1]
 * Explanation: There is no right interval for [1,4] and [3,4].
 * The right interval for [2,3] is [3,4] since start2 = 3 is the smallest start that is >= end1 = 3.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= intervals.length <= 2 * 10^4
 * 	intervals[i].length == 2
 * 	-10^6 <= starti <= endi <= 10^6
 * 	The start point of each interval is unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-right-interval/
// discuss: https://leetcode.com/problems/find-right-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn first_ge_start(sorted_intervals: &Vec<Vec<i32>>, target : i32) -> i32 {
        let mut low = 0;
        let mut high = (sorted_intervals.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_start = sorted_intervals[mid as usize][0];
            if target <= mid_start {
                if mid == 0 || !target <= sorted_intervals[(mid-1) as usize][0] {
                    return mid;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        -1
    }

    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut original_pos = HashMap::new();        
        for (i, interval) in intervals.iter().enumerate() {
            original_pos.insert(interval[0], i as i32);
        }

        // println!("original intervals: {:?}", intervals);
        // println!("original_pos: {:?}", original_pos);

        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by(|a,b|{a[0].cmp(&b[0])});
        // println!("sorted intervals: {:?}", sorted_intervals);

        let mut result = vec![];
        for (i, interval) in intervals.iter().enumerate() {

            let target = interval[1];
            let result_sorted_idx = Self::first_ge_start(&sorted_intervals, target);

            if result_sorted_idx == -1 {
                result.push(-1);
            } else {
                let target_start = sorted_intervals[result_sorted_idx as usize][0];
                result.push(*original_pos.get(&target_start).unwrap());
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
    fn test_436() {
        assert_eq!(Solution::find_right_interval(vec![vec![3,4], vec![2,3], vec![1,2]]), vec![-1,0,1]);
        assert_eq!(Solution::find_right_interval(vec![vec![4,5], vec![2,3], vec![1,2]]), vec![-1,0,1]);
    }
}
