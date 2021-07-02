/**
 * [6] ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 * 
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 * 
 * string convert(string s, int numRows);
 * 
 *  
 * Example 1:
 * 
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * 
 * Example 2:
 * 
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * 
 * Example 3:
 * 
 * Input: s = "A", numRows = 1
 * Output: "A"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, row_count: i32) -> String {
        if row_count == 1 {return s}
        let row_count : usize = row_count as usize;
        let s : Vec<char> = s.chars().collect();
        let period : usize = 2*(row_count - 1);

        let period_count : usize = s.len() / period + 1;
        let col_count : usize = period_count * (row_count - 1);

        let mut output : Vec<Vec<char>> = vec![vec![' ';col_count];row_count];
        // println!("col_count : {}", col_count);
        let mut char_pos : usize = 0;
        for p in 0..period_count {
            let mut cur_col : usize = p * (row_count - 1);
            let mut cur_row : usize = 0;
            while char_pos < s.len() && (cur_row as usize) < row_count {
                // println!("P1: cur_row={},cur_col={}, char_pos={}, char={}", cur_row, cur_col, char_pos, s[char_pos]);
                output[cur_row][cur_col] = s[char_pos];
                char_pos +=1;
                cur_row +=1;
            }
            
            cur_row = row_count - 2;
            cur_col += 1;
            while char_pos < s.len() && 0 < cur_row   {
                // println!("P2: cur_row={},cur_col={}, char_pos={}, char={}", cur_row, cur_col, char_pos, s[char_pos]);

                output[cur_row][cur_col] = s[char_pos];
                char_pos +=1;
                cur_row -=1;
                cur_col +=1;
            }
        }

        let mut result : Vec<char> = vec![];
        for i in 0..row_count {
            for j in 0..col_count {
                if output[i][j] != ' ' {
                    result.push(output[i][j]);
                }
            }
        }
        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}
