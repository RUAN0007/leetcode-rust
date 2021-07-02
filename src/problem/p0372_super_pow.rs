/**
 * [372] Super Pow
 *
 * Your task is to calculate a^b mod 1337 where a is a positive integer and b is an extremely large positive integer given in the form of an array.
 *  
 * Example 1:
 * Input: a = 2, b = [3]
 * Output: 8
 * Example 2:
 * Input: a = 2, b = [1,0]
 * Output: 1024
 * Example 3:
 * Input: a = 1, b = [4,3,3,8,5,2]
 * Output: 1
 * Example 4:
 * Input: a = 2147483647, b = [2,0,0]
 * Output: 1198
 *  
 * Constraints:
 * 
 * 	1 <= a <= 2^31 - 1
 * 	1 <= b.length <= 2000
 * 	0 <= b[i] <= 9
 * 	b doesn't contain leading zeros.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-pow/
// discuss: https://leetcode.com/problems/super-pow/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MODULAR : i64 = 1337;
impl Solution {
    pub fn pow_mod_abase(base : i64, power : usize) -> i64 {
        let mut r : i64 = 1;
        for _ in 0..power {
            r = (r * base) % MODULAR;
        }
        r
    }
    pub fn helper(a: i64, b: &Vec<i64>, cur_pos : usize) -> i64 {
        if cur_pos == b.len() {return 1}
        let this_num : i64 = b[b.len() - 1 - cur_pos];
        let prev : i64 = Self::helper(a, b, cur_pos + 1);
        let prev : i64 = Self::pow_mod_abase(prev, 10);
        let this_num : i64 = Self::pow_mod_abase(a,this_num as usize);
        (prev * this_num) % MODULAR
    }
    
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        Self::helper(a as i64, &b.into_iter().map(|x|{x as i64}).collect(), 0) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_372() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1,0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4,3,3,8,5,2]), 1);
        assert_eq!(Solution::super_pow(2147483647, vec![2,0,0]), 1198);
    }
}
