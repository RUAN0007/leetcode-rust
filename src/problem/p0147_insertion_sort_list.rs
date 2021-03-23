/**
 * [147] Insertion Sort List
 *
 * Given the head of a singly linked list, sort the list using insertion sort, and return the sorted list's head.
 * The steps of the insertion sort algorithm:
 * <ol>
 * 	Insertion sort iterates, consuming one input element each repetition and growing a sorted output list.
 * 	At each iteration, insertion sort removes one element from the input data, finds the location it belongs within the sorted list and inserts it there.
 * 	It repeats until no input elements remain.
 * </ol>
 * The following is a graphical example of the insertion sort algorithm. The partially sorted list (black) initially contains only the first element in the list. One element (red) is removed from the input data and inserted in-place into the sorted list with each iteration.
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif" style="height:180px; width:300px" />
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/sort1linked-list.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [4,2,1,3]
 * Output: [1,2,3,4]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/sort2linked-list.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [-1,5,3,4,0]
 * Output: [-1,0,3,4,5]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [1, 5000].
 * 	-5000 <= Node.val <= 5000
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/insertion-sort-list/
// discuss: https://leetcode.com/problems/insertion-sort-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted = Some(Box::new(ListNode::new(-10000000)));
        while let Some(mut unsorted_node) = head {
            head = unsorted_node.next.take();

            let mut sorted_node = sorted.as_mut().unwrap();
            while let Some(ref next_node) = sorted_node.next {
                if unsorted_node.val < next_node.val {
                    break
                }
                sorted_node = sorted_node.next.as_mut().unwrap();
            }
            let tmp = sorted_node.next.take();
            unsorted_node.next = tmp;
            sorted_node.next = Some(unsorted_node)
        }

        sorted.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_147() {}
}
