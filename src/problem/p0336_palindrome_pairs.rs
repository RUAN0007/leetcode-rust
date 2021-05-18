/**
 * [336] Palindrome Pairs
 *
 * Given a list of unique words, return all the pairs of the distinct indices (i, j) in the given list, so that the concatenation of the two words words[i] + words[j] is a palindrome.
 *  
 * Example 1:
 * 
 * Input: words = ["abcd","dcba","lls","s","sssll"]
 * Output: [[0,1],[1,0],[3,2],[2,4]]
 * Explanation: The palindromes are ["dcbaabcd","abcddcba","slls","llssssll"]
 * 
 * Example 2:
 * 
 * Input: words = ["bat","tab","cat"]
 * Output: [[0,1],[1,0]]
 * Explanation: The palindromes are ["battab","tabbat"]
 * 
 * Example 3:
 * 
 * Input: words = ["a",""]
 * Output: [[0,1],[1,0]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 5000
 * 	0 <= words[i].length <= 300
 * 	words[i] consists of lower-case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-pairs/
// discuss: https://leetcode.com/problems/palindrome-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
struct TrieNode {
    branches : Vec<Option<Box<TrieNode>>>,
    word_index : Option<usize>, 
    suffix_palindrome_indices : Vec<usize>,
}

fn is_palindrome(word : &Vec<char>) -> bool {
    if word.len() == 0 {return true}
    let mut start = 0usize;
    let mut end = word.len() - 1;
    while start < end {
        if word[start] != word[end] {return false;}
        start += 1;
        end -= 1;
    }
    true
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode{branches: (0..26).map(|_|{None}).collect(), 
    word_index: None, suffix_palindrome_indices: vec![]}
    }

    pub fn add_reversed_word(&mut self, word : &Vec<char>, word_idx : usize, level : usize) {
        let pad : String = (0..level).map(|_|{"    "}).collect();
        println!("{}word={:?}, word_idx={}", pad, word, word_idx);
        if word.len() > 0 {
            if is_palindrome(word) {
                self.suffix_palindrome_indices.push(word_idx);
                println!("{}    is_palindrome", pad);
            }

            let first_char : char = word[0];
            let first_char_pos : usize = (first_char as u8 - 'a' as u8) as usize;
            if self.branches[first_char_pos].is_none() {
                self.branches[first_char_pos] = Some(Box::new(TrieNode::new()));
            }

            let the_rest : Vec<char> = word.iter().skip(1).cloned().collect();
            self.branches[first_char_pos].as_mut().unwrap().add_reversed_word(&the_rest, word_idx, level + 1);
        } else {
            self.word_index = Some(word_idx);
            println!("{}    END", pad);
        }

    }

    pub fn search(&self, word : &Vec<char>, word_idx : usize, results : &mut Vec<Vec<i32>>, level : usize) {
        let pad : String = (0..level).map(|_|{"    "}).collect();
        println!("{}word={:?}, word_idx={}", pad, word, word_idx);
        if let Some(matched_index) = self.word_index {
            // matched_idx == word_idx if the word itself is a palindrome
            // word can be "" if two words are "abc" and "cba", etc. 
            
            if matched_index != word_idx && is_palindrome(word) {
                println!("{}  reversed_word_idx{}", pad, self.word_index.unwrap());
                results.push(vec![word_idx as i32, self.word_index.unwrap() as i32]);
            }
        }

        if word.len() > 0 {
            let first_char : char = word[0];
            let first_char_pos : usize = (first_char as u8 - 'a' as u8) as usize;

            if self.branches[first_char_pos].is_some() {
                let the_rest : Vec<char> = word.iter().skip(1).cloned().collect();
                self.branches[first_char_pos].as_ref().unwrap().search(&the_rest, word_idx, results, level + 1);
            }
        } else {
            println!("{}  suffix_palindrome_indices{:?}", pad, self.suffix_palindrome_indices);

            for &idx in self.suffix_palindrome_indices.iter() {
                results.push(vec![word_idx as i32, idx as i32]);
            }
        }
    }
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut root = TrieNode::new();
        let mut results : Vec<Vec<i32>> = vec![];
        for (i, word) in words.iter().enumerate() {
            let reversed_chars :Vec<char> = word.chars().rev().collect();
            root.add_reversed_word(&reversed_chars, i, 0);
        }
        println!("=============================================");
        for (i, word) in words.iter().enumerate() {
            let chars : Vec<char> = word.chars().collect();
            root.search(&chars, i, &mut results, 0);
        }
        results
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_336() {
        assert_eq!(Solution::palindrome_pairs(vec_string!["abcd","dcba","lls","s","sssll"]), vec![vec![0,1],vec![1,0],vec![2,4],vec![3,2]]);

        assert_eq!(Solution::palindrome_pairs(vec_string!["bat","tab","cat"]), vec![vec![0,1],vec![1,0]]);

        assert_eq!(Solution::palindrome_pairs(vec_string!["a", ""]), vec![vec![0,1],vec![1,0]]);
    }
}
