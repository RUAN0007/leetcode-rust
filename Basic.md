# Typical Problems
## List
### Traversal
* [234 Palindrome Linked List](src/problem/p0234_palindrome_linked_list.rs)
### Manipulation
* [206 Reverse Linked List](src/problem/p0206_reverse_linked_list.rs)
* [21 Merge Two Sorted Lists](src/problem/p0021_merge_two_sorted_lists.rs)
  * By list heads
* [19 Remove Nth Node From End of List](src/problem/p0019_remove_nth_node_from_end_of_list.rs)
  * By mutable references
### Hard
* [25 Reverse Nodes in k-Group](src/problem/p0025_reverse_nodes_in_k_group.rs)


## Tree
### Preorder/Inorder/Postorder Traversal
* [94. Binary Tree Inorder Traversal](src/problem/p0094_binary_tree_inorder_traversal.rs)
### BFS Traversal
* [107 Binary Tree Level Order Traversal II](src/problem/p0107_binary_tree_level_order_traversal_ii.rs)
### Manipulation
* [105. Construct Binary Tree from Preorder and Inorder Traversal](src/problem/p0105_construct_binary_tree_from_preorder_and_inorder_traversal.rs)
* [106. Construct Binary Tree from Inorder and Postorder Traversal](src/problem/p0106_construct_binary_tree_from_inorder_and_postorder_traversal.rs)
* [108. Convert Sorted Array to Binary Search Tree](src/problem/p0108_convert_sorted_array_to_binary_search_tree.rs)
### Review
* [101. Symmetric Tree](src/problem/p0101_symmetric_tree.rs)
* [222. Count Complete Tree Nodes](src/problem/p0222_count_complete_tree_nodes.rs)
* [236. Lowest Common Ancestor of a Binary Tree](src/problem/p0236_lowest_common_ancestor_of_a_binary_tree.rs)

### Hard
* [99. Recover Binary Search Tree](src/problem/p0099_recover_binary_search_tree.rs)

## Stack
### Parsing
* [726. Number of Atoms](src/problem/p0726_number_of_atoms.rs)
During iteration, think when to update parameters, when to process these parameters and re-reinitialize them, and when to invoke recursively. Note to match parentheses when subtracting subparts for the recursive call. 
### Stack-aided Monotonic Array
Extremely powerful for leftmost/rightmost smaller/greater problems in an array.
* [84. Largest Rectangle in Histogram](src/problem/p0084_largest_rectangle_in_histogram.rs)
* [907. Sum of Subarray Minimums](src/problem/p0907_sum_of_subarray_minimums.rs)
### Hard
* [84. Largest Rectangle in Histogram](src/problem/p0084_largest_rectangle_in_histogram.rs)


## Recursions

# Combination and Permutation
* [31. Next Permutation](src/problem/p0031_next_permutation.rs)
(Find the last increasing adjacent pair, the first of which is indexed with k. If k not exists, reverse the entire array. Otherwise, find the greatest l such that arr[l]>arr[k]. Swap l and k, and reverse arr[k+1..])
## 

# DP 
## 01 Knapsack
## Infinite Knapsack
## Classics

## Other Hard
* [42. Trapping Rain Water](src/problem/p0042_trapping_rain_water.rs)
# [Collected Template](src/problem/p0000_template.rs)
* Data structure:
    * BtreeMap
    * BtreeSet
    * HashMap
    * HashSet
    * Vector
    * Iterator
    * Heap
    * Stack
    * Queue
* Bit Operation
* Primitives
    * Option
    * Typed/Untyped structs
    * User Input
* Algorithms
    * Binary Search
    * Union Find (Map-based & vector-based)

# Tricks
Promient Problems
713 Element count in window sliding. 
399 str Union Find

Unsolved:
[803. Bricks Falling When Hit](src/problem/p0803_bricks_falling_when_hit.rs)
[126. Word Ladder II](src/problem/p0126_word_ladder_ii.rs)

# Resources

# Note
* LRU(146) and LFU(460) has to be implemented in PYTHON, as only possible to encode in unsafe rust. 