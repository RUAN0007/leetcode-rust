/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" style="width: 200px; height: 162px;" />
 *  
 * Example 1:
 * 
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 * 
 * Example 2:
 * 
 * Input: digits = ""
 * Output: []
 * 
 * Example 3:
 * 
 * Input: digits = "2"
 * Output: ["a","b","c"]
 * 
 *  
 * Constraints:
 * 
 * 	0 <= digits.length <= 4
 * 	digits[i] is a digit in the range ['2', '9'].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(results : &mut Vec<String>, tmp : &mut Vec<char>, digits : &Vec<char>, letters : &Vec<Vec<char>>) {
        if tmp.len() == digits.len() {
            results.push(tmp.iter().clone().collect());
            return;
        }

        let cur_pos : usize = tmp.len();
        let digit : usize = (digits[cur_pos] as u8 - '0' as u8) as usize;
        for &letter in letters[digit].iter() {
            tmp.push(letter);
            Self::helper(results, tmp, digits, letters);
            tmp.pop();
        }

    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits : Vec<char> = digits.chars().collect();
        if digits.len() == 0 {return vec![];}
        let mut results : Vec<String> = vec![];
        let mut tmp : Vec<char> = vec![];
        let letters : Vec<Vec<char>> = vec![
            vec![], // 0
            vec![], // 1
            vec!['a', 'b', 'c'], // 2
            vec!['d', 'e', 'f'], // 3
            vec!['g', 'h', 'i'], // 4
            vec!['j', 'k', 'l'], // 5
            vec!['m', 'n', 'o'], // 6
            vec!['p', 'q', 'r', 's'], // 7
            vec!['t', 'u', 'v'], // 8
            vec!['w', 'x', 'y', 'z'], // 9
        ];

        Self::helper(&mut results, &mut tmp, &digits, &letters);
        results
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["cf", "af", "bf", "cd", "ce", "ad", "ae", "bd", "be"]
        );
    }
}
