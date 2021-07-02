/**
 * [241] Different Ways to Add Parentheses
 *
 * Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: expression = "2-1-1"
 * Output: [0,2]
 * Explanation:
 * ((2-1)-1) = 0 
 * (2-(1-1)) = 2
 * 
 * Example 2:
 * 
 * Input: expression = "2*3-4*5"
 * Output: [-34,-14,-10,-10,10]
 * Explanation:
 * (2*(3-(4*5))) = -34 
 * ((2*3)-(4*5)) = -14 
 * ((2*(3-4))*5) = -10 
 * (2*((3-4)*5)) = -10 
 * (((2*3)-4)*5) = 10
 * 
 *  
 * Constraints:
 * 
 * 	1 <= expression.length <= 20
 * 	expression consists of digits and the operator '+', '-', and '*'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/different-ways-to-add-parentheses/
// discuss: https://leetcode.com/problems/different-ways-to-add-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn helper(expression : &Vec<char>, start : usize, end : usize, cache : &mut HashMap<(usize, usize), Vec<i32>>) {
        if let Some(cached) = cache.get(&(start, end)) {
            return
        }

        let mut result : Vec<i32> = vec![];
        let mut acc_num : i32 = 0;
        for i in start..end {
            let c : char = expression[i];
            if c == '+' || c == '-' || c == '*' {
                Self::helper(expression, start, i, cache);
                Self::helper(expression, i+1, end, cache);

                for &left_num in cache.get(&(start, i)).unwrap().iter() {
                    for &right_num in cache.get(&(i+1, end)).unwrap().iter() {
                        if c == '+' {
                            result.push(left_num + right_num);
                        } else if c == '-' {
                            result.push(left_num - right_num);
                        } else {
                            result.push(left_num * right_num);
                        }
                    }
                }
            } else {
                acc_num = 10 * acc_num + (c as u8 - '0' as u8) as i32;
            }
        }
        if result.len() == 0 { result.push(acc_num); }
        // println!("start={}, end={}, result={:?}", start, end, result);

        cache.insert((start, end), result);
    }

    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let expression : Vec<char> = expression.chars().collect();
        let mut cache : HashMap<(usize, usize), Vec<i32>> = HashMap::new();
        let n : usize = expression.len();
        Self::helper(&expression, 0, n, &mut cache);
        cache.get(&(0, n)).unwrap().clone()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_241() {
        assert_eq!(
            Solution::diff_ways_to_compute("2*3".to_owned()),
            vec![6]
        );

        assert_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_owned()),
            vec![-34, -10, -14, -10, 10]
        );
    }
}
