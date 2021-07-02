/**
 * [57] Insert Interval
 *
 * Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
 * You may assume that the intervals were initially sorted according to their start times.
 *  
 * Example 1:
 * 
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 * 
 * Example 2:
 * 
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 * Example 3:
 * 
 * Input: intervals = [], newInterval = [5,7]
 * Output: [[5,7]]
 * 
 * Example 4:
 * 
 * Input: intervals = [[1,5]], newInterval = [2,3]
 * Output: [[1,5]]
 * 
 * Example 5:
 * 
 * Input: intervals = [[1,5]], newInterval = [2,7]
 * Output: [[1,7]]
 * 
 *  
 * Constraints:
 * 
 * 	0 <= intervals.length <= 10^4
 * 	intervals[i].length == 2
 * 	0 <= intervals[i][0] <= intervals[i][1] <= 10^5
 * 	intervals is sorted by intervals[i][0] in ascending order.
 * 	newInterval.length == 2
 * 	0 <= newInterval[0] <= newInterval[1] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-interval/
// discuss: https://leetcode.com/problems/insert-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let mut i : usize = 0;
        let len : usize = intervals.len();
        const START : usize = 0usize;
        const END : usize = 1usize;
        while i < len && intervals[i][END] < new_interval[START] {
            result.push(intervals[i].clone());
            i+=1;
        }
        if i == len {
            // all intervals ends before this new one
            result.push(new_interval);
            return result;
        }

        let new_start : i32 = std::cmp::min(intervals[i][START], new_interval[START]);

        while i < len && intervals[i][START] <= new_interval[END] {
            i+=1;
        }
        let mut new_end : i32 = new_interval[END];
        if 0 < i {
            i-=1; // the idx of the last interval that intervals[i].start <= new_interval[end]
            new_end = std::cmp::max(intervals[i][END], new_interval[END]);
            i+=1;
        }
        // println!("i={}", i);
        result.push(vec![new_start, new_end]);

        while i < len  {
            result.push(intervals[i].clone());
            i+=1;
        }
        result
    }
}

pub struct Interval {}

// problem: https://leetcode.com/problems/insert-interval/
// discuss: https://leetcode.com/problems/insert-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Interval {
    fn new(start : i32, end : i32) -> Vec<i32> {
        vec![start, end]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_57() {
        assert_eq!(
            Solution::insert(
                vec![Interval::new(1, 3), Interval::new(6, 9)],
                Interval::new(2, 5)
            ),
            vec![Interval::new(1, 5), Interval::new(6, 9)]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    Interval::new(1, 2),
                    Interval::new(3, 5),
                    Interval::new(6, 7),
                    Interval::new(8, 10),
                    Interval::new(12, 16)
                ],
                Interval::new(4, 8)
            ),
            vec![
                Interval::new(1, 2),
                Interval::new(3, 10),
                Interval::new(12, 16)
            ]
        );
        assert_eq!(
            Solution::insert(vec![Interval::new(3, 4)], Interval::new(1, 2)),
            vec![Interval::new(1, 2), Interval::new(3, 4)]
        );
        assert_eq!(
            Solution::insert(vec![Interval::new(1, 2)], Interval::new(3, 4)),
            vec![Interval::new(1, 2), Interval::new(3, 4)]
        );
        assert_eq!(
            Solution::insert(vec![Interval::new(1, 2)], Interval::new(2, 3)),
            vec![Interval::new(1, 3)]
        );
        assert_eq!(
            Solution::insert(
                vec![Interval::new(1, 2), Interval::new(3, 4)],
                Interval::new(0, 6)
            ),
            vec![Interval::new(0, 6)]
        );
    }
}
