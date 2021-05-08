/**
 * [282] Expression Add Operators
 *
 * Given a string num that contains only digits and an integer target, return all possibilities to add the binary operators '+', '-', or '*' between the digits of num so that the resultant expression evaluates to the target value.
 *  
 * Example 1:
 * Input: num = "123", target = 6
 * Output: ["1*2*3","1+2+3"]
 * Example 2:
 * Input: num = "232", target = 8
 * Output: ["2*3+2","2+3*2"]
 * Example 3:
 * Input: num = "105", target = 5
 * Output: ["1*0+5","10-5"]
 * Example 4:
 * Input: num = "00", target = 0
 * Output: ["0*0","0+0","0-0"]
 * Example 5:
 * Input: num = "3456237490", target = 9191
 * Output: []
 *  
 * Constraints:
 * 
 * 	1 <= num.length <= 10
 * 	num consists of only digits.
 * 	-2^31 <= target <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/expression-add-operators/
// discuss: https://leetcode.com/problems/expression-add-operators/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backtrack(cur_ops : &mut Vec<char>, results : &mut Vec<String>, nums : &Vec<i32>, target : i32) {
        let pad : String = (0..cur_ops.len()).map(|_|{"  "}).collect();
        // println!("{}cur_opts={:?}",pad, cur_ops);
        if cur_ops.len() == nums.len() - 1 {
            let mut num_stack : Vec<(i32, char)> = vec![(nums[0], '+')];
            for i in 0..cur_ops.len() {
                num_stack.push((nums[i+1], cur_ops[i]));
            }

            let mut nospace_stack : Vec<(i32, char)> = vec![];
            for (&(num, op)) in num_stack.iter() {
                if op == ' ' {
                    let (last_num, last_op) = nospace_stack.pop().unwrap();
                    if last_num == 0 {return;}
                    if let Some(val) = 10i32.checked_mul(last_num) {
                        if let Some(new_num) = val.checked_add(num) {
                            nospace_stack.push((new_num, last_op));
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                } else {
                    nospace_stack.push((num, op));
                }
            }

            let mut nox_stack : Vec<(i32, char)> = vec![];
            for (&(num, op)) in nospace_stack.iter() {
                if op == '*' {
                    let (last_num, last_op) = nox_stack.pop().unwrap();
                    let new_num : i32 = last_num * num;
                    nox_stack.push((new_num, last_op));
                } else {
                    nox_stack.push((num, op));
                }
            }
            // println!("  {}num_stack:{:?}", pad, num_stack);
            // println!("  {}nospace_stack:{:?}", pad, nospace_stack);
            // println!("  {}nox_stack:{:?}", pad, nox_stack);
            // println!("");

            let mut cur_target : i32 = 0;
            for (&(num, op)) in nox_stack.iter() {
                if op == '+' {
                    cur_target += num;
                } else {
                    cur_target -= num;
                }
            }

            if cur_target == target {
                let mut result : String = String::from(nums[0].to_string());
                for i in 0..cur_ops.len() {
                    if cur_ops[i] != ' ' {
                        result.push(cur_ops[i]);
                    }
                    result.push_str(&nums[i+1].to_string());
                }
                results.push(result);
            }
            return;
        }

        let num_idx : usize = cur_ops.len() + 1;

        cur_ops.push('*');
        Self::backtrack(cur_ops, results, nums,  target);
        cur_ops.pop();

        cur_ops.push(' ');
        Self::backtrack(cur_ops, results, nums,  target);
        cur_ops.pop();

        cur_ops.push('+');
        Self::backtrack(cur_ops, results, nums,  target);
        cur_ops.pop();

        cur_ops.push('-');
        Self::backtrack(cur_ops, results, nums,  target);
        cur_ops.pop();

    }


    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let nums : Vec<i32> = num.chars().map(|v|{(v as u8 - '0' as u8) as i32}).collect();
        let mut cur_ops : Vec<char> = vec![];
        let mut result : Vec<String> = vec![];

        Self::backtrack(&mut cur_ops, &mut result, &nums, target);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_282() {
        assert_eq!(Solution::add_operators("123".to_owned(), 6), vec!["1*2*3".to_owned(), "1+2+3".to_owned()]);
        assert_eq!(Solution::add_operators("232".to_owned(), 8), vec!["2*3+2".to_owned(), "2+3*2".to_owned()]);
        assert_eq!(Solution::add_operators("105".to_owned(), 5), vec!["1*0+5".to_owned(), "10-5".to_owned()]);
        assert_eq!(Solution::add_operators("2147483648".to_owned(), -2147483648).len(), 0);
    }
}
