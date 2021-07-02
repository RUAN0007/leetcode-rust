/**
 * [385] Mini Parser
 *
 * Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return the deserialized NestedInteger.
 * Each element is either an integer or a list whose elements may also be integers or other lists.
 *  
 * Example 1:
 * 
 * Input: s = "324"
 * Output: 324
 * Explanation: You should return a NestedInteger object which contains a single integer 324.
 * 
 * Example 2:
 * 
 * Input: s = "[123,[456,[789]]]"
 * Output: [123,[456,[789]]]
 * Explanation: Return a NestedInteger object containing a nested list with 2 elements:
 * 1. An integer containing value 123.
 * 2. A nested list containing two elements:
 *     i.  An integer containing value 456.
 *     ii. A nested list with one element:
 *          a. An integer containing value 789
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 5 * 10^4
 * 	s consists of digits, square brackets "[]", negative sign '-', and commas ','.
 * 	s is the serialization of valid NestedInteger.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/mini-parser/
// discuss: https://leetcode.com/problems/mini-parser/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        // println!("s={}", s);
        let mut chars : Vec<char> = s.chars().collect();
        if chars[0] == '[' {
            assert_eq!(*chars.last().unwrap(), ']');
            // exclude the first and list []
            chars.pop();
            chars.drain(0..1);

            let mut l : Vec<NestedInteger> = vec![];
            // while let Some(comma_pos) = chars.iter().position(|&c|{c==','}) {
            loop {
                let mut in_bracket_count : usize = 0;
                let mut comma_pos : Option<usize> = None;
                for (i, &c) in chars.iter().enumerate() {
                    if c == '[' {
                        in_bracket_count+=1;
                    } else if c == ']' {
                        in_bracket_count-=1;
                    } else if c == ',' && in_bracket_count == 0 {
                        comma_pos = Some(i);
                        break;
                    }
                }
                if let Some(comma_pos) = comma_pos {
                    let mut sub_str : Vec<char> = chars.drain(..comma_pos+1).collect();
                    sub_str.pop(); // pop the comma
                    l.push(Solution::deserialize(sub_str.into_iter().collect()));
                } else {
                    break;
                }
            }
            if chars.len() > 0 {
                l.push(Solution::deserialize(chars.into_iter().collect()));
            }
            NestedInteger::List(l)
        } else {
            NestedInteger::Int(s.parse::<i32>().unwrap())     
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_385() {
        // println!("{:?}", Solution::deserialize("[123,[456,[789]]]".to_owned()));
        println!("{:?}", Solution::deserialize("[123,456,[788,799,833],[[]],10,[]]".to_owned()));
    }
}
