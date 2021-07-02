/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 * Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 *  
 * Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *  
 * Constraints:
 * 
 * 	1 <= num1.length, num2.length <= 200
 * 	num1 and num2 consist of digits only.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add(num1 : &Vec<u8>, num2 : &Vec<u8>) -> Vec<u8> {
        let mut carry : u8 = 0;
        let mut result : Vec<u8> = vec![];
        let mut i : usize = 0;
        while carry != 0 || i < num1.len() || i < num2.len() {
            let mut num1_digit : u8 = 0;
            let mut num2_digit : u8 = 0;
            if i < num1.len() { num1_digit = num1[i]; }
            if i < num2.len() { num2_digit = num2[i]; }
            let this_digit : u8 = (num1_digit + num2_digit + carry) % 10;
            carry = (num1_digit + num2_digit + carry) / 10;

            result.push(this_digit);
            i+=1;
        }
        result
    }

    pub fn mul(num : &Vec<u8>, digit : u8, base : usize) -> Vec<u8> {
        let mut result : Vec<u8> = vec![0u8;base];
        let mut carry : u8 = 0;
        let mut i : usize = 0; 
        while carry != 0 || i < num.len(){
            let mut num_digit : u8 = 0;
            if i < num.len() { num_digit = num[i]; }
            let this_digit : u8 = (num_digit * digit + carry) % 10;
            carry = (num_digit * digit + carry) / 10;

            result.push(this_digit);
            i+=1;
        }
        result
    }

    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" { return "0".to_owned(); }

        let num1 : Vec<u8> = num1.chars().map(|c : char|{c as u8 - '0' as u8}).rev().collect();
        let num2 : Vec<u8> = num2.chars().map(|c : char|{c as u8 - '0' as u8}).rev().collect();
        let mut result : Vec<u8> = vec![];
        // println!("num1={:?}", num1);
        for (i, &num2_digit) in num2.iter().enumerate() {
            let this_mul : Vec<u8> = Self::mul(&num1, num2_digit, i);
            // println!("i={}, this_mul={:?}", i, this_mul);
            result = Self::add(&result, &this_mul);
        }
        result.into_iter().rev().map(|c|{(c + '0' as u8) as char}).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(Solution::multiply("2".to_owned(), "3".to_owned()), "6".to_owned());
        assert_eq!(Solution::multiply("123".to_owned(), "456".to_owned()), "56088".to_owned());
        assert_eq!(Solution::multiply("9133".to_owned(), "0".to_owned()), "0".to_owned());
    }
}
