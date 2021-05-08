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

// problem: https://leetcode.com/problems/word-search-ii/
// discuss: https://leetcode.com/problems/word-search-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
pub struct TrieNode{
    branches : Vec<Option<Box<TrieNode>>>,
    word: Option<String>,
}

impl TrieNode {
    pub fn new_node() -> TrieNode {
        TrieNode{
            branches: (0..26).map(|_| None).collect(),
            word : None,
        }
    }

    pub fn build_trie(words : &Vec<String>) -> TrieNode {
        let mut root = Self::new_node();
        for word in words.iter() {
            let mut node_ptr = &mut root;
            for word_char in word.chars() {
                let pos : usize = (word_char as u8 - 'a' as u8) as usize;
                if node_ptr.branches[pos].is_none() {
                    node_ptr.branches[pos] = Some(Box::new(TrieNode::new_node()));
                }
                node_ptr =  node_ptr.branches[pos].as_mut().unwrap();
            }
            node_ptr.word = Some(word.clone());
        }
        root
    }
}

use std::collections::HashSet;
pub struct Solution {}
impl Solution {
    pub fn track(board: &mut Vec<Vec<char>>, i : i32, j : i32, cur_node : &TrieNode, found : &mut HashSet<String> ) {
        let row_count : i32 = board.len() as i32;
        let col_count : i32 = board[0].len() as i32;
        if cur_node.word.is_some() {
            found.insert(cur_node.word.as_ref().unwrap().clone());
        }

        if (0 <= i  && i < row_count && 0 <= j && j < col_count && board[i as usize][j as usize] != '#' ) {
            let this_char : char = board[i as usize][j as usize];
            let char_idx : usize = (this_char as u8 - 'a' as u8) as usize;
            if cur_node.branches[char_idx].is_some() {
                let next_node : &TrieNode = cur_node.branches[char_idx].as_ref().unwrap();
                board[i as usize][j as usize] = '#';
                Self::track(board, i-1, j, next_node, found);
                Self::track(board, i+1, j, next_node, found);
                Self::track(board, i, j-1, next_node, found);
                Self::track(board, i, j+1, next_node, found);
                board[i as usize][j as usize] = this_char;
            }

        }

    }

    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let trie : TrieNode = TrieNode::build_trie(&words);

        let mut result : HashSet<String> = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::track(&mut board, i as i32, j as i32, &trie, &mut result);
            }
        }
        // let i = 1usize;
        // let j = 3usize;
        // Self::track(&mut board, i as i32, j as i32, &trie, &mut result);
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
        assert_eq!(Solution::find_words(board, words), vec!["oath","eat"]);
    }
}
