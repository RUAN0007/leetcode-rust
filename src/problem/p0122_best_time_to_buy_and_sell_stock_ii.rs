/**
 * [122] Best Time to Buy and Sell Stock II
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 * 
 * Input: prices = [7,1,5,3,6,4]
 * Output: 7
 * Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
 * Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
 * 
 * Example 2:
 * 
 * Input: prices = [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 * Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
 * 
 * Example 3:
 * 
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e., max profit = 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= prices.length <= 3 * 10^4
 * 	0 <= prices[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut last_no_stock_balance : i32 = 0;
        let mut last_with_stock_balance : i32 = -2_147_483_648i32;

        for price in prices.iter() {
            let last_no_stock_balance_cache = last_no_stock_balance;
            last_no_stock_balance = std::cmp::max(last_no_stock_balance, last_with_stock_balance + price);

            last_with_stock_balance = std::cmp::max(last_with_stock_balance, last_no_stock_balance_cache - price)
        }
        last_no_stock_balance
    }

    pub fn max_profit1(prices: Vec<i32>) -> i32 {
        let n : usize = prices.len();
        if n == 0 || n == 1 {return 0;}
        let mut local_mins : Vec<i32> = vec![];
        let mut local_maxs : Vec<i32> = vec![];

        let is_local_min = |i|{ 
            if i==0 {
                prices[i] <= prices[i+1]
            } else if i == n-1 {
                false
            } else {
                prices[i-1] >= prices[i] && prices[i] <= prices[i+1]
            }
        };

        let is_local_max = |i|{ 
            if i==0 {
                false
            } else if i == n-1 {
                prices[i-1] <= prices[i]
            } else {
                prices[i-1] <= prices[i] && prices[i] >= prices[i+1]
            }
        };
        let mut i : usize = 0;
        loop {
            let mut cur_local_min : i32 = 0;
            let mut cur_local_max : i32 = 0;
            while i < n && !is_local_min(i) {
                i+=1;
            }
            if i == n { break;}
            cur_local_min = prices[i];

            i+=1;
            while i < n && !is_local_max(i) {
                i+=1;
            }
            if i == n { break;}
            cur_local_max = prices[i];

            local_mins.push(cur_local_min);
            local_maxs.push(cur_local_max);
        }
        // println!("local_mins={:?}, local_maxs={:?}", local_mins, local_maxs);
        let mut sum : i32 = 0;
        for i in 0..local_maxs.len() {
            sum += local_maxs[i] - local_mins[i];
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_122() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![2, 2, 5]), 3);
    }
}
