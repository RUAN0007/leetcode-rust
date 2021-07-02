/**
 * [211] Design Add and Search Words Data Structure
 *
 * Design a data structure that supports adding new words and finding if a string matches any previously added string.
 * Implement the WordDictionary class:
 * 
 * 	WordDictionary() Initializes the object.
 * 	void addWord(word) Adds word to the data structure, it can be matched later.
 * 	bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.
 * 
 *  
 * Example:
 * 
 * Input
 * ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
 * [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
 * Output
 * [null,null,null,null,false,true,true,true]
 * Explanation
 * WordDictionary wordDictionary = new WordDictionary();
 * wordDictionary.addWord("bad");
 * wordDictionary.addWord("dad");
 * wordDictionary.addWord("mad");
 * wordDictionary.search("pad"); // return False
 * wordDictionary.search("bad"); // return True
 * wordDictionary.search(".ad"); // return True
 * wordDictionary.search("b.."); // return True
 * 
 *  
 * Constraints:
 * 
 * 	1 <= word.length <= 500
 * 	word in addWord consists lower-case English letters.
 * 	word in search consist of  '.' or lower-case English letters.
 * 	At most 50000 calls will be made to addWord and search.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-add-and-search-words-data-structure/
// discuss: https://leetcode.com/problems/design-add-and-search-words-data-structure/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
struct TrieNode {
    branches : [Option<Box<TrieNode>>;26],
    exists : bool,
}

impl TrieNode {
    pub fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        TrieNode{
            branches : [NONE;26],
            exists : false
        }
    }
    pub fn insert(&mut self, word : &str) {
        if word == "" { self.exists = true;  return; }
        let first_char : char = word.chars().next().unwrap();
        let char_pos : usize = (first_char as u8 - 'a' as u8) as usize;
        if self.branches[char_pos].is_none() {
            self.branches[char_pos] = Some(Box::new(TrieNode::new()));
        }
        self.branches[char_pos].as_mut().unwrap().insert(&word[1..]);
    }

    pub fn search(&self, word : &str) -> bool {
        if word == "" {
            if self.exists {
                return true;
            } else {
                return false;
            }
        }
        let first_char : char = word.chars().next().unwrap();
        if first_char == '.' {
            for i in 0..26 {
                if self.branches[i].is_none() {continue}
                if self.branches[i].as_ref().unwrap().search(&word[1..]) {
                    return true;
                }
            }
            false
        } else {
            let char_pos : usize = (first_char as u8 - 'a' as u8) as usize;
            if self.branches[char_pos].is_none() {
                false
            } else {
                self.branches[char_pos].as_ref().unwrap().search(&word[1..])
            }
        }
    }
}

struct WordDictionary {
    trie : TrieNode
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
       WordDictionary{trie: TrieNode::new()} 
    }
    
    fn add_word(&mut self, word: String) {
        self.trie.insert(&word); 
    }
    
    fn search(&self, word: String) -> bool {
       self.trie.search(&word)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_211() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_owned());
        dict.add_word("dad".to_owned());
        dict.add_word("mad".to_owned());
        assert_eq!(dict.search("pad".to_owned()), false);
        assert_eq!(dict.search("bad".to_owned()), true);
        assert_eq!(dict.search(".ad".to_owned()), true);
        assert_eq!(dict.search("da.".to_owned()), true);
    }
}
