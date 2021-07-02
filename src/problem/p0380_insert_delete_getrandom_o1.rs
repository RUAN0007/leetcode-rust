/**
 * [380] Insert Delete GetRandom O(1)
 *
 * Implement the RandomizedSet class:
 * 
 * 	RandomizedSet() Initializes the RandomizedSet object.
 * 	bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
 * 	bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
 * 	int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
 * 
 * You must implement the functions of the class such that each function works in average O(1) time complexity.
 *  
 * Example 1:
 * 
 * Input
 * ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
 * [[], [1], [2], [2], [], [1], [2], []]
 * Output
 * [null, true, false, true, 2, true, false, 2]
 * Explanation
 * RandomizedSet randomizedSet = new RandomizedSet();
 * randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
 * randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
 * randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
 * randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
 * randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
 * randomizedSet.insert(2); // 2 was already in the set, so return false.
 * randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= val <= 2^31 - 1
 * 	At most 2 * 10^5 calls will be made to insert, remove, and getRandom.
 * 	There will be at least one element in the data structure when getRandom is called.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-delete-getrandom-o1/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
struct RandomizedSet {
    val2pos : HashMap<i32, usize>,
    values : Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
       Self{val2pos : HashMap::new(), values: vec![]} 
    }
    
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.val2pos.get(&val).is_none() {
            let pos : usize = self.values.len();
            self.val2pos.insert(val, pos);
            self.values.push(val);
            true
        } else {
            false
        }
        
    }
    
    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
       if let Some(removed_pos)  = self.val2pos.remove(&val) {
           let last_val : i32 = self.values.pop().unwrap();
           if removed_pos != self.values.len() {
               self.val2pos.insert(last_val, removed_pos);
               self.values[removed_pos] = last_val;
           }
           true
       } else {
           false
       }
    }
    
    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let rand_idx : usize = rng.gen::<usize>() % self.values.len(); 
        self.values[rand_idx]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_380() {
    }
}
