/**
 * [341] Flatten Nested List Iterator
 *
 * You are given a nested list of integers nestedList. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.
 * Implement the NestedIterator class:
 * 
 * 	NestedIterator(List<NestedInteger> nestedList) Initializes the iterator with the nested list nestedList.
 * 	int next() Returns the next integer in the nested list.
 * 	boolean hasNext() Returns true if there are still some integers in the nested list and false otherwise.
 * 
 * Your code will be tested with the following pseudocode:
 * 
 * initialize iterator with nestedList
 * res = []
 * while iterator.hasNext()
 *     append iterator.next() to the end of res
 * return res
 * 
 * If res matches the expected flattened list, then your code will be judged as correct.
 *  
 * Example 1:
 * 
 * Input: nestedList = [[1,1],2,[1,1]]
 * Output: [1,1,2,1,1]
 * Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].
 * 
 * Example 2:
 * 
 * Input: nestedList = [1,[4,[6]]]
 * Output: [1,4,6]
 * Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nestedList.length <= 500
 * 	The values of the integers in the nested list is in the range [-10^6, 10^6].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flatten-nested-list-iterator/
// discuss: https://leetcode.com/problems/flatten-nested-list-iterator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

use std::collections::VecDeque;
struct NestedIterator {
    nested_list : VecDeque<NestedInteger>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self{nested_list : nested_list.into_iter().collect()} 
    }
    
    fn next(&mut self) -> i32 {
        self.expand();
        if !self.has_next() {return 0;}
        if let Some(NestedInteger::Int(val)) = self.nested_list.pop_front() {
            val
        } else {
            panic!("The front must be an int due to the previous expand...");
        }
    }

    fn expand(&mut self) {
        // Assume has_next() is true
        while let Some(first_nested_int) = self.nested_list.get(0) {
            match(first_nested_int) {
                NestedInteger::Int(_)=>{break;}
                NestedInteger::List(_) => {
                    if let Some(NestedInteger::List(first_nested_list)) = self.nested_list.pop_front() {
                        for nested_int in first_nested_list.into_iter().rev() {
                            self.nested_list.push_front(nested_int);
                        }
                    } else {

                    }
                }
            }
            // if let NestedInteger::List(nested_list) = self.nested_list.pop_front().unwrap() {
            //     for nested_int in nested_list.into_iter().rev() {
            //         self.nested_list.push_front(nested_int);
            //     }
            // }  {
            //     panic!("Must be a list due to whiel condition");
            // }
        }

    }
    
    fn has_next(&mut self) -> bool {
        self.expand();
        self.nested_list.len() != 0
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_341() {
        let nested = vec![NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]), NestedInteger::Int(2), NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)])];

        let mut iterator = NestedIterator::new(nested);

        let mut res = vec![];
        while iterator.has_next() {
            res.push(iterator.next());
        }
        assert_eq!(res, vec![1,1,2,1,1]);
    }
}
