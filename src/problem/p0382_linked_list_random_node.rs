/**
 * [382] Linked List Random Node
 *
 * Given a singly linked list, return a random node's value from the linked list. Each node must have the same probability of being chosen.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/16/getrand-linked-list.jpg" style="width: 302px; height: 62px;" />
 * Input
 * ["Solution", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom"]
 * [[[1, 2, 3]], [], [], [], [], []]
 * Output
 * [null, 1, 3, 2, 2, 3]
 * Explanation
 * Solution solution = new Solution([1, 2, 3]);
 * solution.getRandom(); // return 1
 * solution.getRandom(); // return 3
 * solution.getRandom(); // return 2
 * solution.getRandom(); // return 2
 * solution.getRandom(); // return 3
 * // getRandom() should return either 1, 2, or 3 randomly. Each element should have equal probability of returning.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the linked list will be in the range [1, 10^4].
 * 	-10^4 <= Node.val <= 10^4
 * 	At most 10^4 calls will be made to getRandom.
 * 
 *  
 * Follow up:
 * 
 * 	What if the linked list is extremely large and its length is unknown to you?
 * 	Could you solve this efficiently without using extra space?
 * 
 */
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/linked-list-random-node/
// discuss: https://leetcode.com/problems/linked-list-random-node/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
struct Solution {
    head : Option<Box<ListNode>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self{head: head} 
    }
    
    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut cur : &Option<Box<ListNode>> = &self.head;
        let mut count : usize = 0;
        let mut target : i32 = 0;
        while cur.is_some() {
            count += 1;
            if rng.gen_range(0.0, 1.0) < 1.0 / (count as f64) {
                target = cur.as_ref().unwrap().val;
            }

            cur = &cur.as_ref().unwrap().next;
        }
        target
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_382() {
    }
}
