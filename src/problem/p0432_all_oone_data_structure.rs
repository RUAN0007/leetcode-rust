/**
 * [432] All O`one Data Structure
 *
 * Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.
 * Implement the AllOne class:
 * 
 * 	AllOne() Initializes the object of the data structure.
 * 	inc(String key) Increments the count of the string key by 1. If key does not exist in the data structure, insert it with count 1.
 * 	dec(String key) Decrements the count of the string key by 1. If the count of key is 0 after the decrement, remove it from the data structure. It is guaranteed that key exists in the data structure before the decrement.
 * 	getMaxKey() Returns one of the keys with the maximal count. If no element exists, return an empty string "".
 * 	getMinKey() Returns one of the keys with the minimum count. If no element exists, return an empty string "".
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
 * [[], ["hello"], ["hello"], [], [], ["leet"], [], []]
 * Output
 * [null, null, null, "hello", "hello", null, "hello", "leet"]
 * Explanation
 * AllOne allOne = new AllOne();
 * allOne.inc("hello");
 * allOne.inc("hello");
 * allOne.getMaxKey(); // return "hello"
 * allOne.getMinKey(); // return "hello"
 * allOne.inc("leet");
 * allOne.getMaxKey(); // return "hello"
 * allOne.getMinKey(); // return "leet"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= key.length <= 10
 * 	key consists of lowercase English letters.
 * 	It is guaranteed that for each call to dec, key is existing in the data structure.
 * 	At most 3 * 10^4 calls will be made to inc, dec, getMaxKey, and getMinKey.
 * 
 *  
 * Follow up: Could you apply all the operations in O(1) time complexity?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/all-oone-data-structure/
// discuss: https://leetcode.com/problems/all-oone-data-structure/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
use std::collections::BTreeMap;

struct AllOne {
    freq_list : BTreeMap<usize, Vec<String>>, 
    str2freq_pos : HashMap<String, (usize, usize)>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    /** Initialize your data structure here. */
    fn new() -> Self {
        AllOne{freq_list : BTreeMap::new(), str2freq_pos : HashMap::new()}
    }
    
    /** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
    fn inc(&mut self, key: String) {
        let mut prev_frq : usize = 0;
        if let Some(&(frq, pos)) = self.str2freq_pos.get(&key) {
            prev_frq = frq;
            self.remove(frq, pos);
        }
        self.insert(prev_frq + 1, &key);
    }

    fn insert(&mut self, frq : usize, key : &String) {
        if !self.freq_list.contains_key(&frq) {
            self.freq_list.insert(frq, vec![]);
        }

        let pos : usize = self.freq_list[&frq].len();
        self.freq_list.get_mut(&frq).unwrap().push(key.clone());
        self.str2freq_pos.insert(key.clone(), (frq, pos));
    }

    fn remove(&mut self, frq : usize, pos : usize) {
        let removed_key : String = self.freq_list[&frq][pos].clone();
        self.str2freq_pos.remove(&removed_key);
        if pos == self.freq_list[&frq].len() - 1 {
            let last_key : String = self.freq_list.get_mut(&frq).unwrap().pop().unwrap();
            if self.freq_list[&frq].len() == 0 {
                self.freq_list.remove(&frq);
            }
            // last key
        } else {
            // substitute the removed position with the last key
            let last_key : String = self.freq_list.get_mut(&frq).unwrap().pop().unwrap();
            self.str2freq_pos.insert(last_key.clone(), (frq, pos));
            self.freq_list.get_mut(&frq).unwrap()[pos] = last_key;
        }
    }
    
    /** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
    fn dec(&mut self, key: String) {
        let (frq, pos) = self.str2freq_pos[&key];
        self.remove(frq, pos);
        if frq > 1 {
            self.insert(frq - 1, &key);
        }
    }
    
    /** Returns one of the keys with maximal value. */
    fn get_max_key(&self) -> String {
        if let Some((frq, str_list)) = self.freq_list.range(std::ops::RangeFull).next_back() {
            str_list.last().unwrap().clone()
        } else {
            "".to_owned()
        }
    }
    
    /** Returns one of the keys with Minimal value. */
    fn get_min_key(&self) -> String {
        if let Some((frq, str_list)) = self.freq_list.range(std::ops::RangeFull).next() {
            str_list.last().unwrap().clone()
        } else {
            "".to_owned()
        }
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    fn helper(label : String, ops : Vec<String>, parameters : Vec<Vec<String>>, outputs : Vec<String>) {
        let mut obj = AllOne::new();
        for i in 0..ops.len() {
            let op : String = ops[i].clone();
            let output : String = outputs[i].clone();

            if op == "inc" {
                let key : String = parameters[i][0].clone();
                obj.inc(key);
            } else if op == "dec" {
                let key : String = parameters[i][0].clone();
                obj.dec(key);
            } else if op == "getMaxKey" {
                let result : String = obj.get_max_key();
                if output != result {
                    panic!("Inconsistent output: i={}, op={}, output={}, actual={}", i, op, output, result);
                }
            } else if op == "getMinKey" {
                let result : String = obj.get_min_key();
                if output != result {
                    panic!("Inconsistent output: i={}, op={}, output={}, actual={}", i, op, output, result);
                }

            }
        }
    }

    #[test]
    fn test_432() {
        helper("test1".to_owned(), vec_string!["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"], 
        vec![vec_string![], vec_string!["hello"], vec_string!["hello"], vec_string![], vec_string![], vec_string!["leet"], vec_string![], vec_string![]], 
        vec_string!["null", "null", "null", "hello", "hello", "null", "hello", "leet"]);
    }
}
