/**
 * [420] Strong Password Checker
 *
 * A password is considered strong if the below conditions are all met:
 * 
 * 	It has at least 6 characters and at most 20 characters.
 * 	It contains at least one lowercase letter, at least one uppercase letter, and at least one digit.
 * 	It does not contain three repeating characters in a row (i.e., "...aaa..." is weak, but "...aa...a..." is strong, assuming other conditions are met).
 * 
 * Given a string password, return the minimum number of steps required to make password strong. if password is already strong, return 0.
 * In one step, you can:
 * 
 * 	Insert one character to password,
 * 	Delete one character from password, or
 * 	Replace one character of password with another character.
 * 
 *  
 * Example 1:
 * Input: password = "a"
 * Output: 5
 * Example 2:
 * Input: password = "aA1"
 * Output: 3
 * Example 3:
 * Input: password = "1337C0d3"
 * Output: 0
 *  
 * Constraints:
 * 
 * 	1 <= password.length <= 50
 * 	password consists of letters, digits, dot '.' or exclamation mark '!'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strong-password-checker/
// discuss: https://leetcode.com/problems/strong-password-checker/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        // each insertion or replacement can reduce a triplet.
        let mut triplet_count : usize = 0;

        // streak with 3n consecutive char, each streak requires for a single char removal to reduce a triplet. Then it results into a n3_2 streak
        let mut n3_0_streak_count : usize = 0;

        // streak with 3n+1 consecutive char, each streak requires for two-char removal to reduce a triplet. Then it results into a n3_2 streak. 
        let mut n3_1_streak_count : usize = 0;

        // streak with 3n+2 consecutive char, each streak allows for three-char removal to reduce a triplet. Then it is still a n3_2 streak. 
        let mut n3_2_streak_count : usize = 0;
        let mut missed_type_count : usize = 0;

        let len : usize = password.len();
        let mut needed_lower : usize = 1;
        let mut needed_upper : usize = 1;
        let mut needed_digit : usize = 1;

        for c in password.chars() {
            if c.is_ascii_digit() {
                needed_digit = 0;
            }
            if c.is_ascii_lowercase() {
                needed_lower = 0; 
            }
            if c.is_ascii_uppercase() {
                needed_upper = 0;
            }
        }

        // For each needed types, there need 1 insertion/replacement. 
        let needed_types = needed_digit + needed_lower + needed_upper;

        let mut last_char : char = ' '; // any char not exist in password
        let mut last_len : usize = 0;
        for c in password.chars() {
            if c == last_char {
                last_len += 1;
            } else {
                if last_len >= 3 {
                    println!("last_len={}, last_char={}", last_len, last_char);
                    triplet_count += last_len / 3;
                    if last_len % 3 == 0 {
                        n3_0_streak_count+=1;
                    } else if last_len % 3 == 1 {
                        n3_1_streak_count+=1;
                    } else if last_len % 3 == 2 {
                        n3_2_streak_count +=1;
                    }
                }
                last_len = 1;
                last_char = c;
            }
        }

        if last_len >= 3 {
            println!("last_len={}, last_char={}", last_len, last_char);
            triplet_count += last_len / 3;
            if last_len % 3 == 0 {
                n3_0_streak_count+=1;
            } else if last_len % 3 == 1 {
                n3_1_streak_count+=1;
            } else if last_len % 3 == 2 {
                n3_2_streak_count +=1;
            }
        }

        println!("len={}, missed_type_count: {}, triplet_count: {}, n3_0_count: {}, n3_1_count:{}, n3_2_count:{}", len, missed_type_count, triplet_count, n3_0_streak_count, n3_1_streak_count, n3_2_streak_count);

        let min_len : usize = 6;
        let max_len : usize = 20;
        if len < min_len {
            let needed_insertion : usize = min_len - len;
            // Char insertion to achieve all
            //   * Reach the min_len
            //   * Reduce a triplet
            //   * Add a missed type.
            *[needed_insertion, triplet_count, needed_types].iter().max().unwrap() as i32
        } else if len <= max_len {
            // Char replacement to achieve both
            //   * Reduce a triplet
            //   * Add a missed type.
            *[triplet_count, needed_types].iter().max().unwrap() as i32
        } else {
            let needed_removal : usize = len - max_len;
            let mut removal_quota : usize = len - max_len;
            // Attempt to reduce triplet by char removal with the provided quota, rather than insertion/replacement. 
            // Start from n3_0 streak, then n3_1, n3_2, as n3_0 streak require the least char removal to reduce a triplet. 

            let n3_0_reduced_triplet : usize = std::cmp::min(removal_quota, n3_0_streak_count);
            triplet_count -= n3_0_reduced_triplet;
            removal_quota -= n3_0_reduced_triplet;

            let n3_1_reduced_triplet : usize = std::cmp::min(removal_quota / 2, n3_1_streak_count);
            triplet_count -= n3_1_reduced_triplet;
            removal_quota -= n3_1_reduced_triplet * 2;

            // the remaining triplets are all from n3_2 streak, since the above typed streaks have all been transformed into n3_2 streaks. 
            let n3_2_reduced_triplet : usize = std::cmp::min(removal_quota / 3, triplet_count);
            triplet_count -= n3_2_reduced_triplet; // the remaining triplet that have to reduced by replacement. 
            removal_quota -= n3_2_reduced_triplet * 3;

            needed_removal as i32 + *[triplet_count, needed_types].iter().max().unwrap() as i32
        }

    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_420() {
        assert_eq!(Solution::strong_password_checker("bbaaaaaaaaaaaaaaacccccc".to_owned()), 8);
        // assert_eq!(Solution::strong_password_checker("aaa111".to_owned()), 2);
        // assert_eq!(Solution::strong_password_checker("a".to_owned()), 5);
        // assert_eq!(Solution::strong_password_checker("aA1".to_owned()), 3);
        // assert_eq!(Solution::strong_password_checker("1337C0d3".to_owned()), 0);
    }
}
