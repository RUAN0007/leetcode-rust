/**
 * [381] Insert Delete GetRandom O(1) - Duplicates allowed
 *
 * Implement the RandomizedCollection class:
 * 
 * 	RandomizedCollection() Initializes the RandomizedCollection object.
 * 	bool insert(int val) Inserts an item val into the multiset if not present. Returns true if the item was not present, false otherwise.
 * 	bool remove(int val) Removes an item val from the multiset if present. Returns true if the item was present, false otherwise. Note that if val has multiple occurrences in the multiset, we only remove one of them.
 * 	int getRandom() Returns a random element from the current multiset of elements (it's guaranteed that at least one element exists when this method is called). The probability of each element being returned is linearly related to the number of same values the multiset contains.
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
 * [[], [1], [1], [2], [], [1], []]
 * Output
 * [null, true, false, true, 2, true, 1]
 * Explanation
 * RandomizedCollection randomizedCollection = new RandomizedCollection();
 * randomizedCollection.insert(1);   // return True. Inserts 1 to the collection. Returns true as the collection did not contain 1.
 * randomizedCollection.insert(1);   // return False. Inserts another 1 to the collection. Returns false as the collection contained 1. Collection now contains [1,1].
 * randomizedCollection.insert(2);   // return True. Inserts 2 to the collection, returns true. Collection now contains [1,1,2].
 * randomizedCollection.getRandom(); // getRandom should return 1 with the probability 2/3, and returns 2 with the probability 1/3.
 * randomizedCollection.remove(1);   // return True. Removes 1 from the collection, returns true. Collection now contains [1,2].
 * randomizedCollection.getRandom(); // getRandom should return 1 and 2 both equally likely.
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= val <= 2^31 - 1
 * 	At most 10^5 calls will be made to insert, remove, and getRandom.
 * 	There will be at least one element in the data structure when getRandom is called.
 * 
 *  
 * Follow up: Could you implement the functions of the class with each function works in average O(1) time?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
use std::collections::HashSet;

struct RandomizedCollection {
    nums : Vec<i32>,
    positions : HashMap<i32, HashSet<usize>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {

    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedCollection{nums : vec![], positions : HashMap::new()}        
    }
    
    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let pos : usize = self.nums.len();
        self.nums.push(val);
        if self.positions.contains_key(&val) {
            self.positions.get_mut(&val).unwrap().insert(pos);
            true
        } else {
            self.positions.insert(val, [pos].iter().cloned().collect());
            false
        }
    }
    
    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        // println!("nums={:?}, pos={:?}", self.nums, self.positions);
        if self.positions.contains_key(&val) {
            // tentatively remove the last number
            let last_num : i32 = self.nums.pop().unwrap();
            let last_num_pos : usize = self.nums.len();
            assert!(self.positions.get_mut(&last_num).unwrap().remove(&last_num_pos));


            if val != last_num {
                // find any position of val to be removed. 
                let removed_val_pos : usize = *self.positions[&val].iter().next().unwrap();
                assert!(self.positions.get_mut(&val).unwrap().remove(&removed_val_pos));

                if self.positions[&val].len() == 0 { self.positions.remove(&val); }

                // copy the last number to the removed position.
                self.nums[removed_val_pos] = last_num;
                self.positions.get_mut(&last_num).unwrap().insert(removed_val_pos);
            } else if self.positions[&last_num].len() == 0 { 
                self.positions.remove(&last_num); 
            }
            true
        } else {
            false
        }
    }
    
    /** Get a random element from the collection. */

    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let pos : usize = rng.gen::<usize>() % self.nums.len();
        self.nums[pos] 
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    fn help(test_label : String, ops : &Vec<String>, parameters : &Vec<Vec<i32>>) {
        let mut rc : RandomizedCollection = RandomizedCollection::new();
        for (i, op) in ops.iter().enumerate() {
            if op.eq("insert") {
                rc.insert(parameters[i][0]);
            } else if op.eq("remove") {
                rc.remove(parameters[i][0]);
            } else if op.eq("getRandom") {
                rc.get_random();
            }
        }
    }

    #[test]
    fn test_381() {
        // let ops1 : Vec<String> = vec_string!["RandomizedCollection","insert","insert","insert","getRandom","remove","getRandom"];
        // let params1 : Vec<Vec<i32>> = vec![vec![],vec![1],vec![1],vec![2],vec![],vec![1],vec![]];
        // help("test1".to_owned(), &ops1, &params1);

        // let ops2 : Vec<String> = vec_string!["RandomizedCollection","insert","remove","insert","remove","getRandom","getRandom","getRandom","getRandom","getRandom","getRandom","getRandom","getRandom","getRandom","getRandom"];
        // let params2 : Vec<Vec<i32>> = vec![vec![],vec![0],vec![0],vec![-1],vec![0],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
        // help("test2".to_owned(), &ops2, &params2);


        let ops3 : Vec<String> = vec_string!["RandomizedCollection","insert","insert","insert","getRandom","remove","getRandom"];

        


        let params3 : Vec<Vec<i32>> = vec![vec![],vec![1],vec![1],vec![2],vec![],vec![1],vec![]];
        help("test3".to_owned(), &ops3, &params3);
    }
}
