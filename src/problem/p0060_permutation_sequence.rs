/**
 * [60] Permutation Sequence
 *
 * The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 * <ol>
 * 	"123"
 * 	"132"
 * 	"213"
 * 	"231"
 * 	"312"
 * 	"321"
 * </ol>
 * Given n and k, return the k^th permutation sequence.
 *  
 * Example 1:
 * Input: n = 3, k = 3
 * Output: "213"
 * Example 2:
 * Input: n = 4, k = 9
 * Output: "2314"
 * Example 3:
 * Input: n = 3, k = 1
 * Output: "123"
 *  
 * Constraints:
 * 
 * 	1 <= n <= 9
 * 	1 <= k <= n!
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-sequence/
// discuss: https://leetcode.com/problems/permutation-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_helper(factorial : &Vec<usize>, nums : &mut Vec<char>, k : usize, n : usize) -> Vec<char> {
        // Optional Short-cut to avoid recursive. 
        // if k == 0 {
        //     return nums.iter().filter(|&&c|{c!='0'}).cloned().collect();
        // }
        if n == 0 {return vec![];}
        let i : usize = n - 1;
        let idx : usize = k / factorial[i];
        let c_ptr : &mut char = nums.iter_mut().filter(|c|{**c!='0'}).nth(idx).unwrap();
        let mut result : String = c_ptr.to_string();

        let mut result : Vec<char> = vec![*c_ptr];
        *c_ptr = '0'; // Mark this num/char has been taken
        let next_k = k % factorial[i];
        result.extend(Self::get_helper(factorial, nums, next_k, n - 1));
        return result;
    }

    pub fn get_permutation_backup(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut nums : Vec<char> = vec![];
        for i in 1..=n {
            nums.push(('1' as u8 + (i-1) as u8) as char);
        }
        let mut k : usize = (k - 1) as usize;// make the idex starting from 0
        let mut factorial : Vec<usize> = vec![1;10];
        for i in 1..=9 {
            factorial[i] = i * factorial[i-1];
        }

        Self::get_helper(&factorial, &mut nums, k, n).iter().collect()
    }

    pub fn recursive(factorials : &mut Vec<i32>, k : i32, nums : &mut Vec<i32>) -> String {
        if nums.iter().filter(|&&x|{x!=-1}).next().is_none() {
            return "".to_owned();
        }

        let last_factorial : i32 = factorials.pop().unwrap();
        let nth_max : i32 = k / last_factorial;
        let this_digit : String = nums.iter().filter(|&&x|{x!=-1}).nth(nth_max as usize).unwrap().to_string();
        *nums.iter_mut().filter(|x|{**x!=-1}).nth(nth_max as usize).unwrap() = -1;

        let k = k % last_factorial;
        this_digit + &Self::recursive(factorials, k % last_factorial, nums)
    }

    pub fn get_permutation(n: i32, k: i32) -> String {
        let k : i32 = k - 1;
        let mut factorials : Vec<i32> = vec![1];
        let mut factorial : i32 = 1;
        for i in 1..n {
            factorial *= i as i32;
            factorials.push(factorial);
        }

        let mut nums : Vec<i32> = vec![];
        for i in (1..=n) {
            nums.push(i as i32);
        }

        Self::recursive(&mut factorials, k, &mut nums)
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_owned());
        assert_eq!(Solution::get_permutation(3, 3), "213".to_owned());
        assert_eq!(Solution::get_permutation(3, 1), "123".to_owned());
        assert_eq!(Solution::get_permutation(3, 2), "132".to_owned());
        assert_eq!(Solution::get_permutation(2, 2), "21".to_owned());
    }
}
