/**
 * [502] IPO
 *
 * Suppose LeetCode will start its IPO soon. In order to sell a good price of its shares to Venture Capital, LeetCode would like to work on some projects to increase its capital before the IPO. Since it has limited resources, it can only finish at most k distinct projects before the IPO. Help LeetCode design the best way to maximize its total capital after finishing at most k distinct projects.
 * You are given n projects where the i^th project has a pure profit profits[i] and a minimum capital of capital[i] is needed to start it.
 * Initially, you have w capital. When you finish a project, you will obtain its pure profit and the profit will be added to your total capital.
 * Pick a list of at most k distinct projects from given projects to maximize your final capital, and return the final maximized capital.
 * The answer is guaranteed to fit in a 32-bit signed integer.
 *  
 * Example 1:
 * 
 * Input: k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
 * Output: 4
 * Explanation: Since your initial capital is 0, you can only start the project indexed 0.
 * After finishing it you will obtain profit 1 and your capital becomes 1.
 * With capital 1, you can either start the project indexed 1 or the project indexed 2.
 * Since you can choose at most 2 projects, you need to finish the project indexed 2 to get the maximum capital.
 * Therefore, output the final maximized capital, which is 0 + 1 + 3 = 4.
 * 
 * Example 2:
 * 
 * Input: k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
 * Output: 6
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= 10^5
 * 	0 <= w <= 10^9
 * 	n == profits.length
 * 	n == capital.length
 * 	1 <= n <= 10^5
 * 	0 <= profits[i] <= 10^4
 * 	0 <= capital[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ipo/
// discuss: https://leetcode.com/problems/ipo/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::BTreeMap;
impl Solution {
    pub fn push(heap : &mut BTreeMap<i32,Vec<usize>>, key : i32, pos : usize){
        heap.entry(key).or_insert(vec![]).push(pos);
    }

    pub fn min(heap : &BTreeMap<i32,Vec<usize>>) -> Option<(i32, usize)> {
        if let Some((&max_key, positions)) = heap.iter().next() {
            Some((max_key, *positions.last().unwrap()))
        } else {
            None
        }

    }

    pub fn pop_max(heap : &mut BTreeMap<i32,Vec<usize>>) -> Option<(i32, usize)> {
        if let Some((&max_key, _)) = heap.iter().next_back() {
            let pos : usize = heap.get_mut(&max_key).unwrap().pop().unwrap();
            if heap[&max_key].len() == 0 {heap.remove(&max_key);}
            Some((max_key, pos))
        } else {
            None
        }
    }

    pub fn pop_min(heap : &mut BTreeMap<i32,Vec<usize>>) -> Option<(i32, usize)> {
        if let Some((&min_key, _)) = heap.iter().next() {
            let pos : usize = heap.get_mut(&min_key).unwrap().pop().unwrap();
            if heap[&min_key].len() == 0 {heap.remove(&min_key);}
            Some((min_key, pos))
        } else {
            None
        }
    }


    pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {

        let mut in_budget_prices: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        let mut out_budget_capitals : BTreeMap<i32, Vec<usize>> = BTreeMap::new();

        for (i, &c) in capital.iter().enumerate() {
            Self::push(&mut out_budget_capitals, c, i);
        }

        while k > 0 {
            while let Some((capital, i)) = Self::min(&out_budget_capitals) {
                if capital <= w {
                    Self::pop_min(&mut out_budget_capitals);
                    Self::push(&mut in_budget_prices, profits[i], i);
                } else {
                    break;
                }
            }
            // println!("in_budget_prices={:?}, out_budget_capitals={:?}", in_budget_prices, out_budget_capitals);

            if let Some((profit, i)) = Self::pop_max(&mut in_budget_prices) {
                w += profit;
                k-=1;
            } else {
                break
            }
        }
        w
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_502() {
        assert_eq!(Solution::find_maximized_capital(2, 0, vec![1,2,3], vec![0,1,1]), 4);
        assert_eq!(Solution::find_maximized_capital(3, 0, vec![1,2,3], vec![0,1,2]), 6);
    }
}
