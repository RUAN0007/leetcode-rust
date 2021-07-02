# Typical Problems
## List
### Traversal
```
# Length computation
let mut node_ptr : &ListNode = head.as_ref().unwrap();
while let Some(ref next) = node_ptr.next {
    node_ptr = next;
    l+=1;
}
```
* [234 Palindrome Linked List](src/problem/p0234_palindrome_linked_list.rs)
### Manipulation
* [206 Reverse Linked List](src/problem/p0206_reverse_linked_list.rs)
* [21 Merge Two Sorted Lists](src/problem/p0021_merge_two_sorted_lists.rs)
  * By list heads
* [19 Remove Nth Node From End of List](src/problem/p0019_remove_nth_node_from_end_of_list.rs)
* [92. Reverse Linked List II](src/problem/p0092_reverse_linked_list_ii.rs)
  * By mutable references
### Hard
* [25 Reverse Nodes in k-Group](src/problem/p0025_reverse_nodes_in_k_group.rs)


## Tree
### Preorder/Inorder/Postorder Traversal
* [94. Binary Tree Inorder Traversal](src/problem/p0094_binary_tree_inorder_traversal.rs)
* [Preorder](https://www.geeksforgeeks.org/iterative-preorder-traversal/) and [In-order](https://www.geeksforgeeks.org/iterative-preorder-traversal/) Traversal with Stack
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
Employ to compute a min lexicographically ordered string or numbers with fixed digits,  with certain actions satisfied. 
* [316. Remove Duplicate Letters](src/problem/p0316_remove_duplicate_letters.rs)
* [321. Create Maximum Number](src/problem/p0312_burst_balloons.rs)


## Recursions
Hint: provide a _level_ parameter in each recursive call. Prefix `let level_padding : String = (0..level).map(|_|{"  "}).collect();` in each print message for a clear format. 
### Path Searching
* [79. Word Search](src/problem/p0079_word_search.rs)
* [212. Word Search II](src/problem/p0212_word_search_ii.rs)

### Backtrack
A general backtrack template is provided in the [template](src/problem/p0000_template.rs). This can help the following problems: 
* [78. Subsets](src/problem/p0078_subsets.rs)
* [40. Combination Sum](src/problem/p0039_combination_sum.rs)
* [77. Combinations](src/problem/p0077_combinations.rs)
* [131. Palindrome Partitioning](src/problem/p0131_palindrome_partitioning.rs)

## Permutation
A general permutation template is provided in the [template](src/problem/p0000_template.rs). This can help the following problems: 
* [31. Next Permutation](src/problem/p0031_next_permutation.rs)
(Find the last increasing adjacent pair, the first of which is indexed with k. If k not exists, reverse the entire array. Otherwise, find the greatest l such that arr[l]>arr[k]. Swap l and k, and reverse arr[k+1..])

## Dynamic Programming
### Knapsack
Can use DP only if the states are enumerable, e.g, the target sum is integer. 
Otherwise, must apply the recursion. 
### Bounded
Elements can NOT be reused. 
* [416. Partition Equal Subset Sum](src/problem/p0416_partition_equal_subset_sum.rs)
* [494. Target Sum](src/problem/p0494_target_sum.rs)
### Unbounded
Elements can be reused. 
* [322. Coin Change](src/problem/p0322_coin_change.rs)
* [518. Coin Change 2](src/problem/p0518_coin_change_ii.rs)
### Generalized Approach
```
// result[i][j] represent the result to reach state j only with the first i elements. i=0 implies no elements considered. 
Init result[element_count+1][state_count]

for i = 1..=element_count: 
  for j = 0..=state_count:
    // Bounded
    this_element = element[i-1]
    if j < this_element {
      Derive result[i][j] from result[i-1][j]
    } else if bounded {
      Derive result[i][j] from result[i-1][j-this_element]
    } else {
      Derive result[i][j] from result[i][j-this_element]
    }
result[element_count][state_count]
```

### Classics
* [300. Longest Increasing Subsequence](src/problem/p0300_longest_increasing_subsequence.rs)
### 2D
* [72. Edit Distance](src/problem/p0072_edit_distance.rs)
* [718. Maximum Length of Repeated Subarray](src/problem/p0718_maximum_length_of_repeated_subarray.rs) 
* [1092. Shortest Common Supersequence](src/problem/p1092_shortest_common_supersequence.rs)
* [1143. Longest Common Subsequence](src/problem/p1143_longest_common_subsequence.rs)
* [Others](https://www.techiedelight.com/top-10-dynamic-programming-problems/)

## Binary Search
Refer to the [template](src/problem/p0000_template.rs) for the classic binary search problem, such as _first greater element_, and etc. 
```
// Assume the target must exist
low = 0i32; // inclusive and can be negative
high = nums.len() as i32 - 1;// inclusive and can be negative

while low < high {
  let mid = (low + high) / 2;
  // three branches can be merged. 
  if nums[mid]==target{
  // may additional test whether it is the first and the last to be true
  // If meeting condition: return
  // else shrink the range:
    high = mid or mid-1
    low = mid + 1; NEVER low = mid, otherwise the loop is infinite. 
  } else if nums[mid] < target {
    // similar as above
  } else {
    // similar as above
  }
}
low

// Assume the target may not exist
low = 0i32; // inclusive and can be negative
high = nums.len() as i32 - 1;// inclusive and can be negative

while low <= high {
  let mid = (low + high) / 2;
  // three branches can be merged. 
  if nums[mid]==target{
  // may additional test whether it is the first and the last to be true
  // If meeting condition: return
  // else shrink the range:
    high = mid - 1; or
    low = mid + 1; NEVER high/low = mid, otherwise the loop is infinite. 
  } else if nums[mid] < target {
    // similar as above
  } else {
    // similar as above
  }
}
```

### Rotated Sorted List
```
// A general approach
let left = 0;
let right = nums.len() - 1;
while low <= high {
  // check mid_num
  if left_num < mid_num {
    // adjust the ranges given that [left, mid] is sorted.
    // Assume [left, mid] is not sorted, then suppose nums[left=0]=0 nums[left+1=-1] a[mid=2]=1, this array can never be rotated to be sorted. 
  } else if left_num > mid_num {
    // adjust the ranges given that [mid, right] is sorted. 
  } else {
    // increment left given that left_num = mid_num
  }
}
-1
```
* [33. Search in Rotated Sorted Array](src/problem/p0033_search_in_rotated_sorted_array.rs)
* [153. Find Minimum in Rotated Sorted Array](src/problem/p0153_find_minimum_in_rotated_sorted_array.rs)
* [81. Search in Rotated Sorted Array II](src/problem/p0081_search_in_rotated_sorted_array_ii.rs)
* [154. Find Minimum in Rotated Sorted Array II](src/problem/p0154_find_minimum_in_rotated_sorted_array_ii.rs)
### Max-min
Suitable for problems where solutions are within in a range and the tentative solution is easy to verify. 
* [410. Split Array Largest Sum](src/problem/p0410_split_array_largest_sum.rs)
* [875. Koko Eating Bananas](src/problem/p0875_koko_eating_bananas.rs)

### H-Index
* [275. H-Index II](src/problem/p0275_h_index_ii.rs)
(Find the smallest i such that citations[i] >= n-i, return n-i)
## Sort
### Bucket Sort
* [164. Maximum Gap](src/problem/p0164_maximun_gap.rs)
* [274. H-Index](src/problem/p0274_h_index.rs)
### Cardinality Sort
* [75. Sort Colors](src/problem/p0075_sort_colors.rs)
### Wiggle Sort
```
# Handy code to produce a mapped index, like [0,2,4,1,3,5] or [0,2,4,1,3]
(0..(n+1)/2).map(|x|{2*x}).collect(); // [0,2,4] when n=5 or 6.
(0..n/2).map(|x|{2*x}).collect(); // [1,3] when n=5, or [1,3,5] when n=6.
In [1,3,0,2,4] or [1,3,5,0,2,4], every consecutive 3 are not adjacent. 
```
* [324. Wiggle Sort II](src/problem/p0324_wiggle_sort_ii.rs)
* [767. Reorganize String](src/problem/p0767_reorganize_string.rs)

## Matrix Traversal
### Spiral
* [54. Spiral Matrix](src/problem/p0054_spiral_matrix.rs)
* [885. Spiral Matrix III](src/problem/p0885_spiral_matrix_iii.rs)
### Diagonal
* [498. Diagonal Traverse](src/problem/p0498_diagonal_traverse.rs)
* [1424. Diagonal Traverse II](src/problem/p1424_diagonal_traverse_ii.rs)

## Bit Operation
### Basics
* [190. Reverse Bits](src/problem/p0190_reverse_bits.rs) (Repeatedly test and set)
* [191. Number of 1 Bits](src/problem/p0191_number_of_1_bits.rs) (Use x&=x-1 to repeatively unset the least significant 1-bit. )
* [693. Binary Number with Alternating Bits](src/problem/p0693_binary_number_with_alternating_bits.rs) (Use x&(x+1)==0 to check whether x=(2^n-1))
* [137. Single Number II](src/problem/p0260_single_number_ii.rs) (Find the single unique elements that appear k times, while the rest with n times. )
  * Implement a cyclic counter with period n for each bit when xoring elements
  * Return i-th bit counter if the i-th bit is set in binary k. 
* [260. Single Number III](src/problem/p0260_single_number_iii.rs) (Find the two unique elements, (x,y) that appear k (k is odd) times, while the rest with n times. )
  * Implement a cyclic counter with period n for each bit when xoring elements
  * Locate any i-th bit counter, which is none-zero.(x,y differs in i-th bit).
  * Separate all elements into two groups by i-th bit and redo p137 to discovery x and y. 
* [29. Divide Two Integers](src/problem/p0029_divide_two_integers.rs)
* [371. Sum of Two Integers](src/problem/p0371_sum_of_two_integers.rs)
* [201. Bitwise AND of Numbers Range](src/problem/p0201_bitwise_and_of_numbers_range.rs)


## Series
### Palindrome
```
NOTE: DP recursion for palindrome
is_palindrome(i,j) |= is_palindrome(k,j-1) && s[k-1] == s[j] for any k
```

* [5. Longest Palindromic Substring](src/problem/p0005_longest_palindromic_substring.rs)
### Stocks Trading
Key recursion: 
```
// no_stock_balances[i][k] represents the max balance at the end of i-th day with at most k txns, conditioned on no stock hold
// with_stock_balances[i][k] represents the max balance at the end of i-th day with at most k txns, conditioned on stocks holded
no_stock_balances[i<=0][*]=0; // initial condition
no_stock_balances[*][k=0]=0; // initial condition
with_stock_balances[*][k=0]=MIN; // imply N.A. to work with the below max operation

// sell
no_stock_balances[i][k] = max(no_stock_balances[i-1][k], with_stock_balances[i-1][k] + prices[i])
// buy
with_stock_balances[i][k] = max(with_stock_balances[i-1][k], no_stock_balances[i-1][k-1] - prices[i])
```

* [121. Best Time to Buy and Sell Stock](src/problem/p0121_best_time_to_buy_and_sell_stock.rs) (k=1)
  * Enumerate k dimension with named variables.
  * Always replace no_stock_balances[i-1][k-1] with 0 as k=1
* [122. Best Time to Buy and Sell Stock II](src/problem/p0122_best_time_to_buy_and_sell_stock_ii.rs) (k=inf)
  * Due inf, no_stock_balances[\*][k]=no_stock_balances[\*][k-1], and similar to with_stock_balances. Hence, the dimension for k can be removed. 
* [188. Best Time to Buy and Sell Stock IV](src/problem/p0188_best_time_to_buy_and_sell_stock_iv.rs) (Arbitrary k)
  * Minor Optimization: since a txn must span at least two days, one day to buy and one day to sell, when k >n/2, it is equivalent to k = inf. 
* [714. Best Time to Buy and Sell Stock with Transaction Fee](src/problem/p0714_best_time_to_buy_and_sell_stock_with_transaction_fee.rs)
  * Similar to [P122](src/problem/p0122_best_time_to_buy_and_sell_stock_ii.rs).
  * Can pay the fee either during the buy or the sell. 
* [309. Best Time to Buy and Sell Stock with Cooldown](src/problem/p0309_best_time_to_buy_and_sell_stock_with_cooldown.rs) (cool down with inf k)
  * with_stock_balances[i][k] = max(with_stock_balances[i-1][k], no_stock_balances[**i-2**][k-1] - prices[i])


## Non-intuitive Medium
* [229. Majority Element II](src/problem/p0229_majority_element_ii.rs)
  * (B-M Majority Vote)
* [238. Product of Array Except Self](src/problem/p0238_product_of_array_except_self.rs)
* [240. Search a 2D Matrix II](src/problem/p0240_search_a_2d_matrix_ii.rs)
* [241. Different Ways to Add Parentheses](src/problem/p0241_different_ways_to_add_parentheses.rs)
* [264. Ugly Number II](src/problem/p0264_ugly_number_ii.rs)
  * DP
* [279. Perfect Squares](src/problem/p0279_perfect_squares.rs)
  * Knapsack DP
* [307. Range Sum Query - Mutable](src/problem/p0307_range_sum_query_mutable.rs)
  * Binary Index Tree, detailed [here](https://www.topcoder.com/thrive/articles/Binary%20Indexed%20Trees#introduction)
    * Range sum query
* [310. Minimum Height Trees](src/problem/p0310_minimum_height_trees.rs)
    * BFS from leaves
* [334. Increasing Triplet Subsequence](src/problem/p0334_increasing_triplet_subsequence.rs)
  * Smart math tricks.
* [341. Flatten Nested List Iterator](src/problem/p0341_flatten_nested_list_iterator.rs)
  * Pre-order Traversal with Stack.
* [365. Water and Jug Problem](src/problem/p0365_water_and_jug_problem.rs)
  * Bézout's identity: if d is a multiple of gcd(x,y), then d can be represented by ax+by (a,b,x,y are integers.)
* [368. Largest Divisible Subset](src/problem/p0368_largest_divisible_subset.rs)
  * 2D DP on Array
* [372. Super Pow](src/problem/p0372_super_pow.rs)
  * Rely on Eulers' Theorem to form a recursion. 
* [376. Wiggle Subsequence](src/problem/p0376_wiggle_subsequence.rs)
  * 1D DP, greedy. 
* [382. Linked List Random Node](src/problem/p0382_linked_list_random_node.rs)
  * [Reservoir Sampling](https://gregable.com/2007/10/reservoir-sampling.html)
    * Select k elements from an unbounded stream with uniform probability. 
* [384. Shuffle an Array](src/problem/p0384_shuffle_an_array.rs)
  * [FY Algorithm](https://www.geeksforgeeks.org/shuffle-a-given-array-using-fisher-yates-shuffle-algorithm/) for random shuffling. 
* [390. Elimination Game](src/problem/p0390_elimination_game.rs)
  * Focus on invariant head element. 
* [395. Longest Substring with At Least K Repeating Characters](src/problem/p0395_longest_substring_with_at_least_k_repeating_characters.rs)
  * Divide and Conquer (Not 2 Pointer). For each recursion, identify a char which can never be included in the substring, due to the insufficient frequency. Use this char as split points and continue on the substring. 

## Hard
* [4. Median of Two Sorted Arrays](src/problem/p0004_median_of_two_sorted_arrays.rs)
  * Off-one error
* [10. Regular Expression Matching](src/problem/p0010_regular_expression_matching.rs)
  * 2D DP
* [23. Merge k Sorted Lists](src/problem/p0023_merge_k_sorted_lists.rs)
  * Divide and Conquer
* [25. Reverse Nodes in k-Group](src/problem/p0025_reverse_nodes_in_k_group.rs)
  * Recursion
* [30. Substring with Concatenation of All Words](src/problem/p0030_substring_with_concatenation_of_all_words.rs)
  * Iterative String comparison
* [37. Sudoku Solver](src/problem/p0037_sudoku_solver.rs)
  * Recursion
* [41. First Missing Positive](src/problem/p0041_first_missing_positive.rs)
  * Bitset to mark for int presence.
* [42. Trapping Rain Water](src/problem/p0042_trapping_rain_water.rs)
  * Math Modeling
* [44	Wildcard Matching](src/problem/p0044_wildcard_matching.rs)
  * 2D DP
* [51. N-Queens](src/problem/p0051_n_queens.rs)
* [52. N-Queens II](src/problem/p0052_n_queens_ii.rs)
  * Recursion
* [60. Permutation Sequence](src/problem/p0060_permutation_sequence.rs)
  * Off-one error
* [65. Valid Number](src/problem/p0065_valid_number.rs)
  * Engineeringly Complex
* [68. Text Justification](src/problem/p0068_text_justification.rs)
  * Engineeringly Complex
* [72. Edit Distance](src/problem/p0072_edit_distance.rs)
  * 2D DP
* [76. Minimum Window Substring](src/problem/p0076_minimum_window_substring.rs)
  * 2-Pointer Approach
* [84. Largest Rectangle in Histogram](src/problem/p0084_largest_rectangle_in_histogram.rs)
  * Monotonic Stack.
* [85. Maximal Rectangle](src/problem/p0085_maximal_rectangle.rs)
  * Similar to [84. Largest Rectangle in Histogram](src/problem/p0084_largest_rectangle_in_histogram.rs), solved with monotonic stack. 
* [87. Scramble String](src/problem/p0087_scramble_string.rs)
  * Bottom-up approach with the increment on the substring length
  * Top-down with memoization
* [115. Distinct Subsequences](src/problem/p0115_distinct_subsequences.rs)
  * 2D DP. 
* [123. Best Time to Buy and Sell Stock III](src/problem/p0123_best_time_to_buy_and_sell_stock_iii.rs)
  * As above
* [124. Binary Tree Maximum Path Sum](src/problem/p0124_binary_tree_maximum_path_sum.rs)
  * Bottom-up Recursion.
* [127. Word Ladder II](src/problem/p0126_word_ladder_ii.rs)
  * TODO: timeout
* [127. Word Ladder](src/problem/p0127_word_ladder.rs)
  * BFS
* [132. Palindrome Partitioning II](src/problem/p0132_palindrome_partitioning_ii.rs)
  * 2D DP on 1D array (0 <=i<j<len)
* [135. Candy](src/problem/p0135_candy.rs)
  * Smart tricks
* [140. Word Break II](src/problem/p0140_word_break_ii.rs)
  * Recursion
* [149. Max Points on a Line](src/problem/p0149_max_points_on_a_line.rs)
  * Smart tricks: for each point Pi, count other points which share the identical slope with respect to Pi. 
* [154	Find Minimum in Rotated Sorted Array II](src/problem/p0154_find_minimum_in_rotated_sorted_array_ii.rs)
  * As above.
* [164.  Maximum Gap](src/problem/p0164_maximum_gap.rs)
  * Bucket Sort
* [174.	Dungeon Game](src/problem/p0174_dungeon_game.rs)
  * Bottom-up 2D DP, similar to [62. Unique Paths](src/problem/p0062_unique_paths.rs)
* [188. Best Time to Buy and Sell Stock IV](src/problem/p0188_best_time_to_buy_and_sell_stock_iv.rs)
  * As above
* [212. Word Search II](src/problem/p0212_word_search_ii.rs)
  * Build and employ the trie during DFS traversal. 
* [214. Shortest Palindrome](src/problem/p0214_shortest_palindrome.rs)
  * Find the longest palindrome from the first char.
* [218. The Skyline Problem](src/problem/p0218_the_skyline_problem.rs)
  * Collect critical points, which the top-left and top-right point of each rectangle. 
  * Sort the critical points based on point x-coordinate, left or right, and height. (Details commented in code. )
  * Iterate the critical points:
    * If top-left: 
      * Add the corresponding rectangle to active ones
      * If it increases the max height of the active rectangle: update result
    * IF top-right: 
      * Remove the corresponding rectangle in active ons
      * If it reduces the max height, update the result
  (Active rectangles can be maintained as a max-heap. )
* [224. Basic Calculator](src/problem/p0224_basic_calculator.rs)
  * State machine with Recursion.
* [233. Number of Digit One](src/problem/p0233_number_of_digit_one.rs)
  * Smart Tricks to count Digit One in each digit position. 
* [239. Sliding Window Maximum](src/problem/p0239_sliding_window_maximum.rs)
  * Max-heap
* [273.  Integer to English Words ](src/problem/p0273_integer_to_english_words.rs)
  * Engineeringly Complex
* [282. Expression Add Operators](src/problem/p0282_expression_add_operators.rs)
  * Leverage stacks to compute the expression with operation precedence, e.g, multiplication is over addition. 
  * Concatenation (no-op) treated as the top-prior.
* [295. Find Median from Data Stream](src/problem/p0295_find_median_from_data_stream.rs)
  * Min-max heap. 
* [297.  Serialize and Deserialize Binary Tree](src/problem/p0297_serialize_and_deserialize_binary_tree.rs)
  * Preorder Traversal with the null marker
* [301. Remove Invalid Parentheses](src/problem/p0301_remove_invalid_parentheses.rs)
  * Recursion with duplicates.
  * For each collected results after one round of recursion, do another round of recursion. 
* [312. Burst Balloons](src/problem/p0312_burst_balloons.rs)
  * Top-down DP with tricks: start recursion from the last bursted balloon, so that the left and right balloon are known. 
  * Similar to [87. Scramble String](src/problem/p0087_scramble_string.rs)
* [315. Count of Smaller Numbers After Self](src/problem/p0315_count_of_smaller_numbers_after_self.rs)
  * During the merge sort, count the surpassed right numbers for each left one. 
* [321. Create Maximum Number](src/problem/p0312_burst_balloons.rs)
  * Two tricky sub-problems:
    * Given an array of digits, return k of them in order, to form the greatest number. Use stacks. 
    * Given two array of digits, merge them while keeping their relative order, to form the greatest number. 
* [327. Count of Range Sum](src/problem/p0327_count_of_range_sum.rs)
  * Prepare a vector of prefix sum S
  * For each i, count j > i s.t S\[j\]-S\[i\] within the range
    * Leverage the MergeSort, similar to [315. Count of Smaller Numbers After Self](src/problem/p0315_count_of_smaller_numbers_after_self.rs).
* [329. Longest Increasing Path in a Matrix](src/problem/p0329_longest_increasing_path_in_a_matrix.rs
  * 2D DP with Memoization.
* [330. Patching Array](src/problem/p0330_patching_array.rs)
  * Smart Tricks by Recursion: Assume the previous i number can attain \[0,next_miss]),
    * If num[i] <= next_miss:the range can be augment to [0, next_miss+num[i]) by considering num[i].
    * Else:pad the array with next_muss to augment into [0, next_miss*2)
* [335. Self Crossing](src/problem/p0335_self_crossing.rs)
  * Assume edge i to be first crossed, enumerate three canonical scenarios, in which the edge is crossed by i+4,i+5,and i+6.
  * Relate the scenario with the edge length conditions.
* [336. Palindrome Pairs](src/problem/p0336_palindrome_pairs.rs)
  * Build a trie for the reversed strings
* [352. Data Stream as Disjoint Intervals](src/problem/p0352_data_stream_as_disjoint_intervals.rs)
  * Ordered BTree Map
* [354. Russian Doll Envelopes](src/problem/p0354_russian_doll_envelopes.rs)
  * Longest Increasing Subsequence
* [363. Max Sum of Rectangle No Larger Than K](src/problem/p0363_max_sum_of_rectangle_no_larger_than_k.rs)
  * Two related subproblems: 
    * Max Sum Submatrix (KaDane's Algorithm)
    * Subarray with the sum no larger than k (Compute during the Merge Sort, similar to [327. Count of Range Sum](src/problem/p0327_count_of_range_sum.rs))
* [381. Insert Delete GetRandom O(1)](src/problem/p0381_insert_delete_getrandom_o1_duplicates_allowed.rs)
  * Trick: a helper `positions : HashMap<i32, HashSet<usize>>` to track the index positions of each value in the array. It facilitates the removal of value in the middle of the array: fill the removed position with the last value in the array and update `positions` accordingly. 
* [391. Perfect Rectangle](src/problem/p0391_perfect_rectangle.rs)
  * Classify each point to a subset of four categories, top-left, top-right, bottom-left and bottom-right. A point can belong to multiple distinct types, but not duplicated types. 
  * Iterate each point for the above classification, validation and identify points in th corner. 
  * Again iterate each point and validate: if on corner, conforms to corner pattern. If interior, conform to T-pattern or X-pattern.
  * Trick: 4 digits to encode the point type and identify pattern. 
* [403. Frog Jump](src/problem/p0403_frog_jump.rs)
  * Incremental approach
* [407. Trapping Rain Water II](src/problem/p0407_trapping_rain_water_ii.rs)
  * The trapped water volume of a unit is determined by the lowest among all the highest units in each path towards the boundary.
  * Leverage a priority queue. 
* [410.	Split Array Largest Sum](src/problem/p0410_split_array_largest_sum.rs)
  * Binary Search with the countable result candidate and easy verification. 
* [420. Strong Password Checker](src/problem/p0420_strong_password_checker.rs)
  * Mathematically tricky. 
* [432. All O`one Data Structure](src/problem/p0432_all_oone_data_structure.rs)
  * Leverage two data structures: 
    * `frq_arrays`, which maps the frequency to the list of elements with the associated frequency. 
    * `key2frq_array_pos`, which maps the element key to the frequency and the array position. 
* [440. K-th Smallest in Lexicographical Order](src/problem/p0440_k_th_smallest_in_lexicographical_order.rs)
  * Leverage a denary tree, as detailed in [here](https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/discuss/92242/ConciseEasy-to-understand-Java-5ms-solution-with-Explaination)
  * Tricks to calculate the number of children nodes within a parent node, without expanding. 
* [446. Arithmetic Slices II - Subsequence](src/problem/p0446_arithmetic_slices_ii_subsequence)
  * 2D DP, with a data structure counts[i][d]. It counts the arithmetic subsequence ending at nums[i] with difference d with length at least 2. 
* [458. Poor Pigs](src/problem/p0458_poor_pigs.rs)
  * Smart tricks explained [here](https://leetcode.com/problems/poor-pigs/discuss/94266/Another-explanation-and-solution)
* [460. LFU Cache](src/problem/p0460_lfu_cache.py)
  * `frq_lists`, which maps the frequency to the list of elements with the associated frequency. 
  * `key2frq_node`, which maps the element key to the frequency and the list node
* [466. Count The Repetitions](src/problem/p0466_count_the_repetitions.rs)
  * Two-pointer approach to check one string is a subsequence of the other
  * Make the process cyclic to accommodate for repetition. 
* [472. Concatenated Words](src/problem/p0472_concatenated_words.rs)
  * 2D DP to test a word is concatenated by others in a dictionary. 
* [479. Largest Palindrome Product]()
  * Too mathematically hard to attempt
* [480. Sliding Window Median](src/problem/p0480_sliding_window_median.rs)
  * Small and big heap/BtreeMap. 
* [483.	Smallest Good Base](src/problem/p0483_smallest_good_base.rs)
  * Mathematical Tricks explained [here](https://leetcode.com/problems/smallest-good-base/discuss/96587/Python-solution-with-detailed-mathematical-explanation-and-derivation)
* [488. Zuma Game](src/problem/p0488_zuma_game.rs)
  * Backtrack for each combination of possible insertion position and chars.
  * Cached to deduplicate. 
* [493. Reverse Pairs](src/problem/p0493_reverse_pairs.rs)
  * Count during the merge sort
* [502. IPO](src/problem/p0502_ipo.rs)
  * Pick the capital-allowed with the highest profit
  * Leverage min-max heap. 
* [514. Freedom Trail](src/problem/p0514_freedom_trail.rs)
  * 2D DP
* [517. Super Washing Machines](src/problem/p0517_super_washing_machines.rs)
  * Mathematical Tricks
* [546. Remove Boxes](src/problem/p0546_remove_boxes.rs)
  * 3D DP (Smart Recursion Formulation!)
* [552. Student Attendance Record II](src/problem/p0552_student_attendance_record_ii.rs)
  * 3D DP (Smart and Generic Recursion Formulation!)
    * 1st dimension on n:# of days
    * 2nd dimension on the # of current absent days. 
    * 3nd dimension on the # of trailing leave days
  * Given the limited cardinality, the last two dimensions can be unrolled as named variables. 

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