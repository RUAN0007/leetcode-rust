/**
 * [393] UTF-8 Validation
 *
 * Given an integer array data representing the data, return whether it is a valid UTF-8 encoding.
 * A character in UTF8 can be from 1 to 4 bytes long, subjected to the following rules:
 * <ol>
 * 	For a 1-byte character, the first bit is a 0, followed by its Unicode code.
 * 	For an n-bytes character, the first n bits are all one's, the n + 1 bit is 0, followed by n - 1 bytes with the most significant 2 bits being 10.
 * </ol>
 * This is how the UTF-8 encoding would work:
 * 
 *    Char. number range  |        UTF-8 octet sequence
 *       (hexadecimal)    |              (binary)
 *    --------------------+---------------------------------------------
 *    0000 0000-0000 007F | 0xxxxxxx
 *    0000 0080-0000 07FF | 110xxxxx 10xxxxxx
 *    0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
 *    0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
 * 
 * Note: The input is an array of integers. Only the least significant 8 bits of each integer is used to store the data. This means each integer represents only 1 byte of data.
 *  
 * Example 1:
 * 
 * Input: data = [197,130,1]
 * Output: true
 * Explanation: data represents the octet sequence: 11000101 10000010 00000001.
 * It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte character.
 * 
 * Example 2:
 * 
 * Input: data = [235,140,4]
 * Output: false
 * Explanation: data represented the octet sequence: 11101011 10001100 00000100.
 * The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes character.
 * The next byte is a continuation byte which starts with 10 and that's correct.
 * But the second continuation byte does not start with 10, so it is invalid.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= data.length <= 2 * 10^4
 * 	0 <= data[i] <= 255
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/utf-8-validation/
// discuss: https://leetcode.com/problems/utf-8-validation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn leading1_count(n : u8) -> usize {
        let mut count : usize = 0;
        for i in (0..8).rev() {
            if n & (1 << i) != 0 {
                count+=1;
            } else {
                break;
            }
        }
        count
    }
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut data : Vec<i32> = data.into_iter().rev().collect();
        while let Some(head) = data.pop() {
            let mut head : u8 = (head & 0xFF) as u8;
            let leading1_count : usize = Self::leading1_count(head) as usize;
            // println!("head={}, leading1_count={}", head, leading1_count);
            if leading1_count == 0 {
                // valid, do nothing 
            } else if leading1_count == 1 || leading1_count > 4 {
                return false;
            } else if data.len() < leading1_count - 1 {
                return false;
            } else {
                for i in 0..(leading1_count-1) {
                    head = (data.pop().unwrap() & 0xFF) as u8;
                    // println!("head={:0b}", head);
                    let digit7_one = (head & (1 << 7)) != 0;
                    let digit6_zero = (head & (1 << 6)) == 0;
                    if !(digit7_one && digit6_zero) {return false}
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_393() {
        assert!(Solution::valid_utf8(vec![197,130,1]));
        assert!(!Solution::valid_utf8(vec![235,140,4]));
        assert!(!Solution::valid_utf8(vec![250,145,145,145,145]));
    }
}
