/**
 * [354] Russian Doll Envelopes
 *
 * You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
 * One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
 * Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
 * Note: You cannot rotate an envelope.
 *  
 * Example 1:
 * 
 * Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
 * Output: 3
 * Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
 * 
 * Example 2:
 * 
 * Input: envelopes = [[1,1],[1,1],[1,1]]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= envelopes.length <= 5000
 * 	envelopes[i].length == 2
 * 	1 <= wi, hi <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/russian-doll-envelopes/
// discuss: https://leetcode.com/problems/russian-doll-envelopes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_last_smaller(lasts : &Vec<i32>, height : i32) -> i32 {
        let mut start : i32 = 0;
        let mut end : i32 = lasts.len() as i32 - 1;
        while start <= end {
            let mid : usize = (start + (end - start) / 2) as usize;
            if lasts[mid] < height {
                if mid == lasts.len() - 1 || !(lasts[mid+1] < height) {
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

    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // sort by ascending width then descending height if equal width
        envelopes.sort_by(|a,b|{
            if a[0] < b[0] {
                std::cmp::Ordering::Less
            } else if a[0] > b[0] {
                std::cmp::Ordering::Greater
            } else if a[1] <= b[1] {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        // longest increasing subsequence with respect to height
        let mut lasts : Vec<i32> = vec![];
        for envelope in envelopes.iter() {
            let height : i32 = envelope[1];
            let last_smaller_pos : i32 = Self::find_last_smaller(&lasts, height);
            let new_pos : usize = (last_smaller_pos + 1) as usize;
            if new_pos == lasts.len() {
                lasts.push(height)
            } else {
                lasts[new_pos] = height;
            }
        }
        lasts.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_354() {
        assert_eq!(Solution::max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]), 3);
        assert_eq!(Solution::max_envelopes(vec![vec![1,1],vec![1,1],vec![1,1]]), 1);
    }
}
