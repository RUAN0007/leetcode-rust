/**
 * [75] Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
 * We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
 *  
 * Example 1:
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 * Example 2:
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 * Example 3:
 * Input: nums = [0]
 * Output: [0]
 * Example 4:
 * Input: nums = [1]
 * Output: [1]
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 300
 * 	nums[i] is 0, 1, or 2.
 * 
 *  
 * Follow up:
 * 
 * 	Could you solve this problem without using the library's sort function?
 * 	Could you come up with a one-pass algorithm using only O(1) constant space?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() == 0 {return;}
        let mut zero_end : i32 = 0; // exclusive
        let mut two_start : i32 = nums.len() as i32 - 1; // exclusive
        let mut i : i32 = 0;
        // invariant before iterations:
        //   all zeros in [0, zero_end)
        //   all ones in [zero_end, i)
        //   all twos in (two_start, END)
        while i <= two_start {
            let num : i32 = nums[i as usize];
            if num == 0 {
                nums.swap(zero_end as usize, i as usize);
                i+=1;
                zero_end+=1;
            } else if num == 1 {
                i+=1;
            } else if num == 2 {
                nums.swap(two_start as usize, i as usize);
                two_start -=1;
            }
        }
    }

    pub fn sort_colors_2(nums: &mut Vec<i32>) {
        let (mut zero_start, mut one_start, mut two_start) = (nums.len(), nums.len(), nums.len());
        while 0 < zero_start {
            println!("===================================");
            println!("From: {:?}", nums);
            println!("zero: {}, one: {}, two: {}", zero_start, one_start, two_start);

            match(nums[0]) {
                0 =>{ 
                    zero_start-=1;
                    nums.swap(0, zero_start);
                },
                1 => {
                    zero_start-=1;
                    one_start-=1;
                    nums.swap(zero_start, one_start);
                    if 0 < zero_start {
                        // still got unprocessed num. 
                        // move it to start for next-round processing
                        nums.swap(0, one_start);
                    }

                },
                2 => {
                    two_start-=1;
                    zero_start-=1;
                    one_start-=1;
                    nums.swap(zero_start, one_start);
                    nums.swap(one_start, two_start);
                    if 0 < zero_start {
                        nums.swap(0, two_start);
                    } 
                },
                _ => {
                    panic!("error");
                }
            };
            println!("To: {:?}", nums);
            println!("===================================\n");
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut vec = vec![
            1, 2, 0, 1, 2, 2, 2, 0, 0, 0, 2, 1, 1, 2, 0, 1, 2, 2, 1, 1, 0,
        ];
        Solution::sort_colors(&mut vec);
        assert_eq!(
            vec,
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2]
        );

        // let mut vec = vec![];
        // Solution::sort_colors(&mut vec);
        // assert_eq!(vec, vec![]);

        let mut vec = vec![2, 2, 2];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![2, 2, 2]);

        let mut vec = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![0,0,1,1,2,2]);
    }
}
