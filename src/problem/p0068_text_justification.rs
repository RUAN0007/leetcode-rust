/**
 * [68] Text Justification
 *
 * Given an array of words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
 * You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
 * Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
 * For the last line of text, it should be left justified and no extra space is inserted between words.
 * Note:
 * 
 * 	A word is defined as a character sequence consisting of non-space characters only.
 * 	Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
 * 	The input array words contains at least one word.
 * 
 *  
 * Example 1:
 * 
 * Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
 * Output:
 * [
 *    "This    is    an",
 *    "example  of text",
 *    "justification.  "
 * ]
 * Example 2:
 * 
 * Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
 * Output:
 * [
 *   "What   must   be",
 *   "acknowledgment  ",
 *   "shall be        "
 * ]
 * Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
 * Note that the second line is also left-justified becase it contains only one word.
 * Example 3:
 * 
 * Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
 * Output:
 * [
 *   "Science  is  what we",
 *   "understand      well",
 *   "enough to explain to",
 *   "a  computer.  Art is",
 *   "everything  else  we",
 *   "do                  "
 * ]
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 300
 * 	1 <= words[i].length <= 20
 * 	words[i] consists of only English letters and symbols.
 * 	1 <= maxWidth <= 100
 * 	words[i].length <= maxWidth
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/text-justification/
// discuss: https://leetcode.com/problems/text-justification/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn left(this_line : &Vec<String>, max_width : usize) -> String {
        let mut r : String = this_line[0].clone();
        for i in 1..this_line.len() {
            r.push(' ');
            r.push_str(&this_line[i]);
        }

        while r.len() < max_width {
            r.push(' ');
        }
        r
    }

    pub fn full(this_line : &Vec<String>, max_width : usize) -> String {
        let char_count : usize = this_line.iter().map(|x|{x.len()}).sum();
        let space_count : usize = max_width - char_count;
        let slot_count : usize = this_line.len() - 1;

        let space_per_slot : usize = space_count / slot_count;
        let mut spaces : Vec<usize> = vec![space_per_slot; slot_count];

        for i in 0..(space_count % slot_count) {
            spaces[i] +=1;
        }
        // println!("this_line = {:?}, char_count={}, spaces={:?}", this_line, char_count, spaces) ;
        let mut r : String = this_line[0].clone();
        for i in 0..slot_count {
            r.push_str(&(0..spaces[i]).map(|i|{' '}).collect::<String>());
            r.push_str(&this_line[i+1].clone());
        }
        r
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut this_line : Vec<String> = vec![words[0].clone()];
        let mut i : usize = 1;
        let n : usize = words.len();
        let mut lines = vec![];

        loop {
            while i < n && this_line.iter().map(|w|{w.len()+1}).sum::<usize>() + words[i].len() <= max_width {
                this_line.push(words[i].clone());
                i+=1;
            }
            if i == n {
                lines.push(Self::left(&this_line, max_width));
                break;
            } else if this_line.len() == 1 {
                lines.push(Self::left(&this_line, max_width));
            } else {
                lines.push(Self::full(&this_line, max_width));
            }
            this_line = vec![words[i].clone()];
            i += 1;
        }

        lines
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_68() {
        assert_eq!(
            Solution::full_justify(
                vec_string![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            vec_string!["This    is    an", "example  of text", "justification.  "]
        );

        assert_eq!(
            Solution::full_justify(
                vec_string!["What", "must", "be", "acknowledgment", "shall", "be"],
                16
            ),
            vec_string!["What   must   be", "acknowledgment  ", "shall be        "]
        );

        assert_eq!(
            Solution::full_justify(
                vec_string![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20
            ),
            vec_string![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  ",
            ]
        );
    }
}
