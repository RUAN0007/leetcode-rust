/**
 * [93] Restore IP Addresses
 *
 * Given a string s containing only digits, return all possible valid IP addresses that can be obtained from s. You can return them in any order.
 * A valid IP address consists of exactly four integers, each integer is between 0 and 255, separated by single dots and cannot have leading zeros. For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses and "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses. 
 *  
 * Example 1:
 * Input: s = "25525511135"
 * Output: ["255.255.11.135","255.255.111.35"]
 * Example 2:
 * Input: s = "0000"
 * Output: ["0.0.0.0"]
 * Example 3:
 * Input: s = "1111"
 * Output: ["1.1.1.1"]
 * Example 4:
 * Input: s = "010010"
 * Output: ["0.10.0.10","0.100.1.0"]
 * Example 5:
 * Input: s = "101023"
 * Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 3000
 * 	s consists of digits only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/restore-ip-addresses/
// discuss: https://leetcode.com/problems/restore-ip-addresses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(result : &mut Vec<String>, tmp : &mut Vec<String>, cur_pos : usize, chars : &Vec<char>) {
        let l : usize = chars.len();
        if cur_pos == l && tmp.len() == 4 {
            result.push(tmp.join("."));
            return;
        } else if cur_pos == l || tmp.len() == 4 {
            return; 
        }

        tmp.push(chars[cur_pos..cur_pos+1].iter().collect());
        Self::helper(result, tmp, cur_pos + 1, chars);
        tmp.pop();

        if chars[cur_pos] != '0' && cur_pos + 1 < l {
            tmp.push(chars[cur_pos..cur_pos+2].iter().cloned().collect());
            Self::helper(result, tmp, cur_pos + 2, chars);
            tmp.pop(); 
        }

        if chars[cur_pos] != '0' && cur_pos + 2 < l {
            let next_3 : String = chars[cur_pos..cur_pos+3].iter().cloned().collect();
            if next_3.parse::<i32>().unwrap() <= 255 {
                tmp.push(next_3);
                Self::helper(result, tmp, cur_pos + 3, chars);
                tmp.pop(); 
            }
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s : Vec<char> = s.chars().collect();
        let mut result : Vec<String> = vec![];
        let mut tmp : Vec<String> = vec![];

        Self::helper(&mut result, &mut tmp, 0, &s);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_93() {
        assert_eq!(Solution::restore_ip_addresses("25525511135".to_owned()), vec!["255.255.11.135","255.255.111.35"]);

        assert_eq!(Solution::restore_ip_addresses("0000".to_owned()), vec!["0.0.0.0"]);

        assert_eq!(Solution::restore_ip_addresses("1111".to_owned()), vec!["1.1.1.1"]);

        assert_eq!(Solution::restore_ip_addresses("010010".to_owned()), vec!["0.10.0.10","0.100.1.0"]);
    }
}
