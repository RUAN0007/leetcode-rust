/**
 * [1648] Sell Diminishing-Valued Colored Balls
 *
 * You have an inventory of different colored balls, and there is a customer that wants orders balls of any color.
 * The customer weirdly values the colored balls. Each colored ball's value is the number of balls of that color you currently have in your inventory. For example, if you own 6 yellow balls, the customer would pay 6 for the first yellow ball. After the transaction, there are only 5 yellow balls left, so the next yellow ball is then valued at 5 (i.e., the value of the balls decreases as you sell more to the customer).
 * You are given an integer array, inventory, where inventory[i] represents the number of balls of the i^th color that you initially own. You are also given an integer orders, which represents the total number of balls that the customer wants. You can sell the balls in any order.
 * Return the maximum total value that you can attain after selling orders colored balls. As the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/jj.gif" style="width: 480px; height: 270px;" />
 * Input: inventory = [2,5], orders = 4
 * Output: 14
 * Explanation: Sell the 1st color 1 time (2) and the 2nd color 3 times (5 + 4 + 3).
 * The maximum total value is 2 + 5 + 4 + 3 = 14.
 * 
 * Example 2:
 * 
 * Input: inventory = [3,5], orders = 6
 * Output: 19
 * Explanation: Sell the 1st color 2 times (3 + 2) and the 2nd color 4 times (5 + 4 + 3 + 2).
 * The maximum total value is 3 + 2 + 5 + 4 + 3 + 2 = 19.
 * 
 * Example 3:
 * 
 * Input: inventory = [2,8,4,10,6], orders = 20
 * Output: 110
 * 
 * Example 4:
 * 
 * Input: inventory = [1000000000], orders = 1000000000
 * Output: 21
 * Explanation: Sell the 1st color 1000000000 times for a total value of 500000000500000000. 500000000500000000 modulo 10^9 + 7 = 21.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= inventory.length <= 10^5
 * 	1 <= inventory[i] <= 10^9
 * 	1 <= orders <= min(sum(inventory[i]), 10^9)
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sell-diminishing-valued-colored-balls/
// discuss: https://leetcode.com/problems/sell-diminishing-valued-colored-balls/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{BTreeMap, HashMap};
impl Solution {
    // pub fn first_eq_pos(inventory: &Vec<i32>, target : i32) -> usize {
    //     let mut low = 0;
    //     let mut high = (inventory.len() - 1) as i32;
    //     while low <= high {
    //         let mid = (low + high) / 2;
    //         if inventory[mid as usize] == target {
    //             if mid == 0 || inventory[(mid-1) as usize] != target {
    //                 return mid as usize;
    //             }
    //             high = mid - 1;
    //         } else if inventory[mid as usize] < target {
    //             low = mid + 1;
    //         } else  {
    //             high = mid - 1;
    //         }
    //     }
    //     panic!("Shall not here");
    // }

    pub fn max_profit(mut inventory: Vec<i32>, orders: i32) -> i32 {
        inventory.sort();
        let mut value = 0i64;
        let mut num_frq = BTreeMap::new();
        let mut max_num = 0i64;
        let mut orders = orders as i64;
        for num in inventory {
            let num = num as i64;
            max_num = std::cmp::max(max_num, num);
            if let Some(frq) = num_frq.get_mut(&num) {
                *frq += 1;
            } else {
                num_frq.insert(num, 1);
            }
        }

        num_frq.insert(0, 0);

        let mut sub_max_val = max_num;
        let mut acc_count = *num_frq.get(&max_num).unwrap();
        num_frq.remove(&max_num);

        for (&val, &count) in num_frq.iter().rev() {

            if acc_count * (sub_max_val - val) <= orders {
                let sum = ((sub_max_val + val + 1) * (sub_max_val - val) / 2 * acc_count);
                value += sum;
                value = value % (1000000000 + 7);

                orders -= acc_count * (sub_max_val - val);
                sub_max_val = val;
                acc_count += count;
            } else {

                let mut cur_max = sub_max_val;
                while acc_count <= orders {
                    value += cur_max * acc_count;
                    value = value % (1000000000 + 7);
                    orders -= acc_count;
                    cur_max-=1;
                }
                value += cur_max * orders;
                value = value % (1000000000 + 7);

                break
            }

        }

        // while 0 < orders {
        //     let max_val = *inventory.last().unwrap();
        //     let max_pos = Self::first_eq_pos(&inventory, max_val);
        //     let diff = if max_pos == 0 {max_val} else {max_val - inventory[max_pos - 1]};
        //     orders -=1;
        //     value = (value + max_val) % (1000000000 + 7) ;
        //     inventory[max_pos]-=1;
            // let count = std::cmp::min(diff, orders);
            // let sum = (max_val + (max_val - count + 1)) * count / 2;
            // println!("inventory: {:?}, orders: {}", inventory, orders);
            // println!("sum: {:?}\n", sum);
            // value += sum % (1000000000 + 7);
            // inventory[max_pos] -= count;
            // orders -= diff; 
        // }

        value as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1648() {
        assert_eq!(Solution::max_profit(vec![2,5], 4), 14);
        assert_eq!(Solution::max_profit(vec![3,5], 6), 19);
        assert_eq!(Solution::max_profit(vec![1000000000], 1000000000), 21);
    }
}
