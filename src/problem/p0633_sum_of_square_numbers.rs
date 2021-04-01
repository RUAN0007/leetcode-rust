/**
 * [633] Sum of Square Numbers
 *
 * Given a non-negative integer c, decide whether there're two integers a and b such that a^2 + b^2 = c.
 *  
 * Example 1:
 * 
 * Input: c = 5
 * Output: true
 * Explanation: 1 * 1 + 2 * 2 = 5
 * 
 * Example 2:
 * 
 * Input: c = 3
 * Output: false
 * 
 * Example 3:
 * 
 * Input: c = 4
 * Output: true
 * 
 * Example 4:
 * 
 * Input: c = 2
 * Output: true
 * 
 * Example 5:
 * 
 * Input: c = 1
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	0 <= c <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-square-numbers/
// discuss: https://leetcode.com/problems/sum-of-square-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut low = 0;
        let mut high = (c as f64).sqrt() as i32 ;
        while low <= high {
            if low * low + high * high == c {
                return true;
            } else if low * low + high * high < c {
                low += 1;
            } else {
                high -= 1;
            }
        }

        false
    }
    
    pub fn judge_square_sum_2(c: i32) -> bool {
        let mut possible_sqrs = vec![];
        let mut i = 1;
        while i*i < c {
            possible_sqrs.push(i*i);
            i+=1;
        }
        let mut complements = HashSet::new();
        for &sqr in &possible_sqrs {
            if sqr <= c / 2 {
                complements.insert(sqr);
            } else {
                break;
            }
        }
        for &sqr in possible_sqrs.iter().rev() {
            if  c / 2 < sqr {
                if complements.contains(&(c-sqr)) {return true;}
            } else {
                break;
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
    fn test_633() {
    }
}
