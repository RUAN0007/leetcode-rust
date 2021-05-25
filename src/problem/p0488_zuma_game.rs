/**
 * [488] Zuma Game
 *
 * Think about Zuma Game. You have a row of balls on the table, colored red(R), yellow(Y), blue(B), green(G), and white(W). You also have several balls in your hand.
 * Each time, you may choose a ball in your hand, and insert it into the row (including the leftmost place and rightmost place). Then, if there is a group of 3 or more balls in the same color touching, remove these balls. Keep doing this until no more balls can be removed.
 * Find the minimal balls you have to insert to remove all the balls on the table. If you cannot remove all the balls, output -1.
 *  
 * Example 1:
 * 
 * Input: board = "WRRBBW", hand = "RB"
 * Output: -1
 * Explanation: WRRBBW -> WRR[R]BBW -> WBBW -> WBB[B]W -> WW
 * 
 * Example 2:
 * 
 * Input: board = "WWRRBBWW", hand = "WRBRW"
 * Output: 2
 * Explanation: WWRRBBWW -> WWRR[R]BBWW -> WWBBWW -> WWBB[B]WW -> WWWW -> empty
 * 
 * Example 3:
 * 
 * Input: board = "G", hand = "GGGGG"
 * Output: 2
 * Explanation: G -> G[G] -> GG[G] -> empty 
 * 
 * Example 4:
 * 
 * Input: board = "RBYYBBRRB", hand = "YRBGB"
 * Output: 3
 * Explanation: RBYYBBRRB -> RBYY[Y]BBRRB -> RBBBRRB -> RRRB -> B -> B[B] -> BB[B] -> empty 
 * 
 *  
 * Constraints:
 * 
 * 	You may assume that the initial row of balls on the table won&rsquo;t have any 3 or more consecutive balls with the same color.
 * 	1 <= board.length <= 16
 * 	1 <= hand.length <= 5
 * 	Both input strings will be non-empty and only contain characters 'R','Y','B','G','W'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zuma-game/
// discuss: https://leetcode.com/problems/zuma-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn update(board : &Vec<char>) -> Vec<char> {
        // recursively remove a consecutive of three
        let mut prev_char : char = ' ';// any char not exists
        let mut prev_len : usize = 0;
        // println!("board={:?}", board);
        for i in 0..board.len() {

            if board[i] == prev_char {
                prev_len+=1;
            } else {
                if prev_len >= 3 {
                    let streak_start_pos : usize = i - prev_len;
                    // println!("start_pos={}, prev_len={}, i={}", streak_start_pos, prev_len, i);
                    let new_board : Vec<char> = board.iter().enumerate().filter(|&(ii, _)| ii < streak_start_pos || ii >= i).map(|(_, v)| v).cloned().collect();

                    // let mut new_board : Vec<char> = board.iter().take(streak_start_pos).skip(prev_len).take(board.len()).cloned().collect();
                    return Self::update(&new_board);
                }

                prev_char = board[i];
                prev_len = 1;
            }
        }

        if prev_len >= 3 {
            let i = board.len();
            let streak_start_pos : usize = board.len() - prev_len;
            // println!("start_pos={}, prev_len={}, i={}", streak_start_pos, prev_len, board.len());
            let new_board : Vec<char> = board.iter().enumerate().filter(|&(ii, _)| ii < streak_start_pos || ii >= i).map(|(_, v)| v).cloned().collect();
            return Self::update(&new_board);
        }
        board.clone()
    }

    pub fn dfs(board : &Vec<char>, hand : &mut HashMap<char, usize>, level : usize, cache : &mut HashMap<String, i32>) -> i32 {
        let mut key : String = board.iter().collect::<String>();
        key.push('_');
        for (&k, &v) in hand.iter() {
            key.push(k);
            key.push_str(&v.to_string());
        }
        if let Some(&cached) = cache.get(&key) {
            return cached;
        }

        let pad : String = (0..level).map(|_|{"   "}).collect();
        // println!("{}board={},hand={:?}", pad, board.iter().collect::<String>(), hand);
        if board.len() == 0 { return 0i32; }

        let mut result : i32 = -1;
        let mut optimal_pos : usize = 100000;
        let mut optimal_new_board_before : String = "".to_owned();
        let mut optimal_new_board : String = "".to_owned();
        let mut optimal_char : char = ' ';
        for insert_pos in 0..=board.len() {
            let hand_chars : Vec<char> = hand.keys().cloned().collect();
            for &hand_char in hand_chars.iter() {
                if hand[&hand_char] == 0 { continue; }
                *hand.get_mut(&hand_char).unwrap()-=1;
                let mut new_board_before : Vec<char> = board.iter().take(insert_pos).cloned().collect();
                new_board_before.push(hand_char);
                new_board_before.extend(board.iter().skip(insert_pos).cloned().collect::<Vec<char>>());

                let new_board : Vec<char> = Self::update(&new_board_before);
                let next_result : i32 = Self::dfs(&new_board, hand, level+1, cache);
                if next_result != -1 {
                    if result == -1 {
                        result = 1 + next_result;
                        optimal_pos = insert_pos;
                        optimal_char = hand_char;
                        optimal_new_board_before = new_board_before.iter().cloned().collect();
                        optimal_new_board = new_board.iter().cloned().collect();

                    } else if 1 + next_result < result {
                        result = 1 + next_result;

                        optimal_pos = insert_pos;
                        optimal_char = hand_char;
                        optimal_new_board = new_board.iter().cloned().collect();
                        optimal_new_board_before = new_board_before.iter().cloned().collect();
                    }
                }
                *hand.get_mut(&hand_char).unwrap()+=1;
            }
        }
        // if result != -1 {
        //     println!("");
        //     println!("{}OPTIMAL: pos={}, char={}, new_board={}, new_board_before={}",pad, optimal_pos, optimal_char, optimal_new_board, optimal_new_board_before);
        //     println!("{}r={}, board={}, hand={:?}",pad, result, board.iter().collect::<String>(), hand);
        // }
        cache.insert(key, result);
        result
    }

    pub fn find_min_step(board: String, hand: String) -> i32 {
        let board : Vec<char> = board.chars().collect();
        let mut hand2 : HashMap<char, usize> = HashMap::new();
        let mut cache : HashMap<String, i32> = HashMap::new();

        for c in hand.chars() {
            *hand2.entry(c).or_insert(0usize)+=1;
        }
        Self::dfs(&board, &mut hand2, 0, &mut cache)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_488() {
        assert_eq!(Solution::update(&"RBYYBBRRRBY".to_owned().chars().collect::<Vec<char>>()), "RB".to_owned().chars().collect::<Vec<char>>());
        assert_eq!(Solution::update(&"WRRBBBW".to_owned().chars().collect::<Vec<char>>()), "WRRW".to_owned().chars().collect::<Vec<char>>());
        assert_eq!(Solution::update(&"WRRRBBBW".to_owned().chars().collect::<Vec<char>>()), "WW".to_owned().chars().collect::<Vec<char>>());
        assert_eq!(Solution::update(&"WWRRRBBBW".to_owned().chars().collect::<Vec<char>>()), "".to_owned().chars().collect::<Vec<char>>());
        assert_eq!(Solution::update(&"WRRBBW".to_owned().chars().collect::<Vec<char>>()), "WRRBBW".to_owned().chars().collect::<Vec<char>>());

        assert_eq!(Solution::find_min_step("WRRBBW".to_owned(), "RB".to_owned()), -1);
        assert_eq!(Solution::find_min_step("WWRRBBWW".to_owned(), "WRBRW".to_owned()), 2);
        assert_eq!(Solution::find_min_step("G".to_owned(), "GGGGG".to_owned()), 2);

        assert_eq!(Solution::find_min_step("RBYYBBRRB".to_owned(), "YRBGB".to_owned()), 3);
    }
}
