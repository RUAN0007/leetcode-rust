/**
 * [212] Word Search II
 *
 * Given an m x n board of characters and a list of strings words, return all words on the board.
 * Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search1.jpg" style="width: 322px; height: 322px;" />
 * Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
 * Output: ["eat","oath"]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search2.jpg" style="width: 162px; height: 162px;" />
 * Input: board = [["a","b"],["c","d"]], words = ["abcb"]
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 12
 * 	board[i][j] is a lowercase English letter.
 * 	1 <= words.length <= 3 * 10^4
 * 	1 <= words[i].length <= 10
 * 	words[i] consists of lowercase English letters.
 * 	All the strings of words are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-search-ii/
// discuss: https://leetcode.com/problems/word-search-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    // TODO: timeout in larger test cases. 
    // Need to optimize with trie. 
    pub fn track(board: &Vec<Vec<char>>, i : i32, j : i32, target : &String, visited: &mut HashSet<(i32,i32)>) -> bool{
        if target.len() == 0 {return true;}
        let target_char : char = target.chars().nth(0).unwrap();
        let row_count : i32 = board.len() as i32;
        let col_count : i32 = board[0].len() as i32;
        let mut found = false;
        if (!visited.contains(&(i,j)) && 0 <= i  && i < row_count && 0 <= j && j < col_count && board[i as usize][j as usize] == target_char) {
            let next_target : String = String::from(&target[1..]);
            visited.insert((i,j));
            found = Self::track(board, i-1, j, &next_target, visited) ||
            Self::track(board, i+1, j, &next_target, visited) ||
            Self::track(board, i, j-1, &next_target, visited) ||
            Self::track(board, i, j+1, &next_target, visited);
            visited.remove(&(i,j));
        }

        found
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                for word in words.iter() {
                    if result.contains(word) {continue;}
                    let mut visited = HashSet::new();
                    if Self::track(&board, i as i32, j as i32, word, &mut visited) {
                        result.insert(word.clone());
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_212() {
        let board = vec![vec!['o','a','a','n'],vec!['e','t','a','e'],vec!['i','h','k','r'],vec!['i','f','l','v']];
        let words = vec!["oath".to_owned(),"pea".to_owned(),"eat".to_owned(),"rain".to_owned()];
        assert_eq!(Solution::find_words(board, words), vec!["eat","oath"]);
    }
}
