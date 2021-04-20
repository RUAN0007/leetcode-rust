/**
 * [726] Number of Atoms
 *
 * Given a chemical formula (given as a string), return the count of each atom.
 * The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.
 * One or more digits representing that element's count may follow if the count is greater than 1. If the count is 1, no digits will follow. For example, H2O and H2O2 are possible, but H1O2 is impossible.
 * Two formulas concatenated together to produce another formula. For example, H2O2He3Mg4 is also a formula.
 * A formula placed in parentheses, and a count (optionally added) is also a formula. For example, (H2O2) and (H2O2)3 are formulas.
 * Given a formula, return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than 1), followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.
 *  
 *  
 * Example 1:
 * 
 * Input: formula = "H2O"
 * Output: "H2O"
 * Explanation: The count of elements are {'H': 2, 'O': 1}.
 * 
 * Example 2:
 * 
 * Input: formula = "Mg(OH)2"
 * Output: "H2MgO2"
 * Explanation: The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.
 * 
 * Example 3:
 * 
 * Input: formula = "K4(ON(SO3)2)2"
 * Output: "K4N2O14S4"
 * Explanation: The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.
 * 
 * Example 4:
 * 
 * Input: formula = "Be32"
 * Output: "Be32"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= formula.length <= 1000
 * 	formula consists of English letters, digits, '(', and ')'.
 * 	formula is always valid.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-atoms/
// discuss: https://leetcode.com/problems/number-of-atoms/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::BTreeMap;

impl Solution {
    pub fn process(last_element : &mut String, last_digit : &mut String, last_parsed : &mut BTreeMap<String, usize>, my_parsed : &mut BTreeMap<String, usize>) {
        let mut count = 1usize;
        if last_digit.len() != 0 {
            count = last_digit.parse::<usize>().unwrap();
            *last_digit = "".to_owned();
        }

        if last_element.len() != 0 {
            *(my_parsed.entry(last_element.clone()).or_insert(0)) +=count;
            *last_element = "".to_owned();
        } else if last_parsed.len() != 0 {
            for (key, val) in last_parsed.iter() {
                *(my_parsed.entry(key.clone()).or_insert(0)) +=(*val*count);
            }
            last_parsed.clear();
        } else {
            // panic!("Can not be both empty...");
            // can be empty at the start
        }

    }
    pub fn helper(formula: String) -> BTreeMap<String, usize> {
        let l = formula.len();
        let mut i = 0usize;
        let mut my_parsed : BTreeMap<String, usize> = BTreeMap::new();
        let mut last_element = "".to_owned();
        let mut last_digit = "".to_owned();
        let mut last_parsed : BTreeMap<String, usize> = BTreeMap::new();
        while i < l {
            let mut cur_char : char = formula.chars().nth(i).unwrap();
            if cur_char.is_ascii_digit() {
                last_digit.push(cur_char);
            } else if 'A' <= cur_char && cur_char <= 'Z'  {
                Self::process(&mut last_element, &mut last_digit, &mut last_parsed, &mut my_parsed);

                last_element.push(cur_char);

            } else if 'a' <= cur_char && cur_char <= 'z'  {
                last_element.push(cur_char);
            } else if cur_char == '(' {
                // process first
                Self::process(&mut last_element, &mut last_digit, &mut last_parsed, &mut my_parsed);
                let mut left_bracket_count = 1;
                let mut sub_formula = "".to_owned();
                loop {
                    i+=1;
                    let sub_char = formula.chars().nth(i).unwrap();
                    if sub_char == ')' {
                        left_bracket_count-=1;
                        if left_bracket_count == 0 {break;}
                    } 

                    if sub_char == '(' {
                        left_bracket_count+=1;
                    }
                    sub_formula.push(formula.chars().nth(i).unwrap());
                }
                last_parsed = Self::helper(sub_formula);
            } else {
                panic!("Shall not encounter {}", cur_char);
            }
            // println!("formula={}, i={}, cur_char={}, last_digit={}, last_element={}, last_parsed={:?}", formula, i, cur_char, last_digit, last_element, last_parsed);
            i+=1;
        }
        Self::process(&mut last_element, &mut last_digit, &mut last_parsed, &mut my_parsed);
        my_parsed
    }

    pub fn count_of_atoms(formula: String) -> String {
        let mut parsed = Self::helper(formula);
        let mut result = "".to_owned();
        for (element, count) in parsed.iter() {
            result.extend(element.clone().chars());
            if *count > 1 {
                result.extend(count.to_string().to_owned().chars());
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_726() {
        assert_eq!(Solution::count_of_atoms("H2O".to_owned()), "H2O".to_owned());
        assert_eq!(Solution::count_of_atoms("Mg(OH)2".to_owned()), "H2MgO2".to_owned());
        assert_eq!(Solution::count_of_atoms("Be32".to_owned()), "Be32".to_owned());
        assert_eq!(Solution::count_of_atoms("K4(ON(SO3)2)2".to_owned()), "K4N2O14S4".to_owned());
    }
}
