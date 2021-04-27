/**
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day, and an integer fee representing a transaction fee.
 * Find the maximum profit you can achieve. You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 * 
 * Input: prices = [1,3,2,8,4,9], fee = 2
 * Output: 8
 * Explanation: The maximum profit can be achieved by:
 * - Buying at prices[0] = 1
 * - Selling at prices[3] = 8
 * - Buying at prices[4] = 4
 * - Selling at prices[5] = 9
 * The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
 * 
 * Example 2:
 * 
 * Input: prices = [1,3,7,5,10,3], fee = 3
 * Output: 6
 * 
 *  
 * Constraints:
 * 
 * 	1 <= prices.length <= 5 * 10^4
 * 	1 <= prices[i] < 5 * 10^4
 * 	0 <= fee < 5 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here


impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut last_no_stock_balance : i32 = 0;
        let mut last_with_stock_balance : i32 = -2_147_483_648i32;

        for price in prices.iter() {
            let last_no_stock_balance_cache = last_no_stock_balance;

            last_no_stock_balance = std::cmp::max(last_no_stock_balance, last_with_stock_balance + price);

            // may lead to the overflow. 
            // last_no_stock_balance = std::cmp::max(last_no_stock_balance, last_with_stock_balance + price - fee); 

            // pay the fee during buying
            last_with_stock_balance = std::cmp::max(last_with_stock_balance, last_no_stock_balance_cache - price - fee);
        }
        last_no_stock_balance
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_714() {
    }
}
