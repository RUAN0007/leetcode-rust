/**
 * [301] Remove Invalid Parentheses
 *
 * Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.
 * Return all the possible results. You may return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: s = "()())()"
 * Output: ["(())()","()()()"]
 * 
 * Example 2:
 * 
 * Input: s = "(a)())()"
 * Output: ["(a())()","(a)()()"]
 * 
 * Example 3:
 * 
 * Input: s = ")("
 * Output: [""]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 25
 * 	s consists of lowercase English letters and parentheses '(' and ')'.
 * 	There will be at most 20 parentheses in s.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-invalid-parentheses/
// discuss: https://leetcode.com/problems/remove-invalid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove(s : &Vec<char>, results : &mut Vec<String>, tmp_selected:&Vec<usize>, cur_pos: usize, last_removed_pos : usize, char_to_push : char, char_to_pop : char, level : usize) {
        let pad : String = (0..level).map(|_|{"   "}).collect();
        // println!("{}tmp_selected:{:?}, cur_pos={}, last_removed_pos={}", pad, tmp_selected, cur_pos, last_removed_pos);
        let n : usize = s.len();
        let mut unmatched_count : usize = 0;
        let mut cur_selected : Vec<usize> = tmp_selected.clone();
        for i in cur_pos..n {
            cur_selected.push(i);

            if s[i] == char_to_push {
                unmatched_count += 1;
            } else if s[i] == char_to_pop && unmatched_count != 0 {
                unmatched_count -=1;
            } else if s[i] == char_to_pop && unmatched_count == 0 {
                // remove a char_to_pop among selected, avoid duplicates. 
                for (pos_idx, &selected_pos) in cur_selected.iter().enumerate() {
                    let mut valid_removed_pos = false;;

                    if selected_pos >= last_removed_pos && s[selected_pos] == char_to_pop {
                        if pos_idx == 0 {
                            valid_removed_pos = true;
                        } else if s[cur_selected[pos_idx-1]] != char_to_pop {
                            valid_removed_pos = true;
                        } else if cur_selected[pos_idx-1] < last_removed_pos {
                            valid_removed_pos = true;
                        }
                    }
                    // println!("{}pos_idx={}, selected_pos={}, valid_to_remove={}", pad, pos_idx, selected_pos, valid_removed_pos);

                    if valid_removed_pos {
                        let next_selected : Vec<usize> = cur_selected.iter().enumerate().filter(|&(idx, _)|{idx != pos_idx}).map(|(_, &pos)|{pos}).collect();
                        Self::remove(s, results, &next_selected, i+1, selected_pos, char_to_push, char_to_pop, level + 1);
                    }
                }
                return;
            }
        }

        let result : String = cur_selected.iter().map(|&pos|{s[pos]}).collect();
        results.push(result);

    }

    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s : Vec<char> = s.chars().collect();
        let mut forward_results : Vec<String> = vec![];
        let tmp_selected : Vec<usize> = vec![];
        Self::remove(&s, &mut forward_results, &tmp_selected, 0, 0, '(',')', 0);

        let mut backward_results : Vec<String> = vec![];
        for forward_result in forward_results.iter() {
            let reversed : Vec<char> = forward_result.chars().rev().collect();
            Self::remove(&reversed, &mut backward_results, &tmp_selected, 0, 0, ')', '(', 0);
        }

        backward_results.iter().map(|x|{x.chars().rev().collect::<String>()}).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_301() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_owned()),
            vec_string!["(())()", "()()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_owned()),
            vec_string!["(a())()", "(a)()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses(")(".to_owned()),
            vec_string![""]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses("))".to_owned()),
            vec_string![""]
        );

        assert_eq!(
            Solution::remove_invalid_parentheses(")d))".to_owned()),
            vec_string!["d"]
        );
    }
}
