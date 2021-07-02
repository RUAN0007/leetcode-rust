/**
 * [386] Lexicographical Numbers
 *
 * Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
 * You must write an algorithm that runs in O(n) time and uses O(1) extra space. 
 *  
 * Example 1:
 * Input: n = 13
 * Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
 * Example 2:
 * Input: n = 2
 * Output: [1,2]
 *  
 * Constraints:
 * 
 * 	1 <= n <= 5 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographical-numbers/
// discuss: https://leetcode.com/problems/lexicographical-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(result : &mut Vec<i32>, tmp : &mut Vec<i32>, limit : i32) -> bool {
        let mut num : i32 = 0; 
        for &digit in tmp.iter() {
            num = 10 * num + digit;
        }

        if num <= limit {
            if num != 0 { result.push(num); }
            if result.len() == limit as usize {
                true
            } else {
                let mut digits : Vec<i32> = (0..=9).collect();
                if tmp.len() == 0 {
                    digits = (1..=9).collect();
                }

                for &digit in digits.iter() {
                    tmp.push(digit);
                    if Self::helper(result, tmp, limit) {
                        return true;
                    }
                    tmp.pop();
                }
                false
            }
        } else {
            false
        }


    }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result : Vec<i32> = vec![];
        let mut tmp : Vec<i32> = vec![];
        Self::helper(&mut result, &mut tmp, n);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_386() {
        assert_eq!(Solution::lexical_order(13),vec![1,10,11,12,13,2,3,4,5,6,7,8,9]);
        assert_eq!(Solution::lexical_order(2),vec![1,2]);
    }
}
