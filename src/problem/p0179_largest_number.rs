/**
 * [179] Largest Number
 *
 * Given a list of non-negative integers nums, arrange them such that they form the largest number.
 * Note: The result may be very large, so you need to return a string instead of an integer.
 *  
 * Example 1:
 * 
 * Input: nums = [10,2]
 * Output: "210"
 * 
 * Example 2:
 * 
 * Input: nums = [3,30,34,5,9]
 * Output: "9534330"
 * 
 * Example 3:
 * 
 * Input: nums = [1]
 * Output: "1"
 * 
 * Example 4:
 * 
 * Input: nums = [10]
 * Output: "10"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-number/
// discuss: https://leetcode.com/problems/largest-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a,b| { 
            let ab : String = a.to_string() + b.to_string().as_str();
            let ba : String = b.to_string() + a.to_string().as_str();
            ab.cmp(&ba)
        });
        let result : String = nums.iter().map(|x|{x.to_string()}).rev().collect();
        if let Some('0') = result.chars().nth(0) {
            "0".to_owned()
        } else {
            result
        }
        // String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_179() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_owned()
        );
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121".to_owned());
    }
}
