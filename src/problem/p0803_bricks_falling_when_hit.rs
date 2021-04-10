/**
 * [803] Bricks Falling When Hit
 *
 * You are given an m x n binary grid, where each 1 represents a brick and 0 represents an empty space. A brick is stable if:
 * 
 * 	It is directly connected to the top of the grid, or
 * 	At least one other brick in its four adjacent cells is stable.
 * 
 * You are also given an array hits, which is a sequence of erasures we want to apply. Each time we want to erase the brick at the location hits[i] = (rowi, coli). The brick on that location (if it exists) will disappear. Some other bricks may no longer be stable because of that erasure and will fall. Once a brick falls, it is immediately erased from the grid (i.e., it does not land on other stable bricks).
 * Return an array result, where each result[i] is the number of bricks that will fall after the i^th erasure is applied.
 * Note that an erasure may refer to a location with no brick, and if it does, no bricks drop.
 *  
 * Example 1:
 * 
 * Input: grid = [[1,0,0,0],[1,1,1,0]], hits = [[1,0]]
 * Output: [2]
 * Explanation: Starting with the grid:
 * [[1,0,0,0],
 *  [<u>1</u>,1,1,0]]
 * We erase the underlined brick at (1,0), resulting in the grid:
 * [[1,0,0,0],
 *  [0,<u>1</u>,<u>1</u>,0]]
 * The two underlined bricks are no longer stable as they are no longer connected to the top nor adjacent to another stable brick, so they will fall. The resulting grid is:
 * [[1,0,0,0],
 *  [0,0,0,0]]
 * Hence the result is [2].
 * 
 * Example 2:
 * 
 * Input: grid = [[1,0,0,0],[1,1,0,0]], hits = [[1,1],[1,0]]
 * Output: [0,0]
 * Explanation: Starting with the grid:
 * [[1,0,0,0],
 *  [1,<u>1</u>,0,0]]
 * We erase the underlined brick at (1,1), resulting in the grid:
 * [[1,0,0,0],
 *  [1,0,0,0]]
 * All remaining bricks are still stable, so no bricks fall. The grid remains the same:
 * [[1,0,0,0],
 *  [<u>1</u>,0,0,0]]
 * Next, we erase the underlined brick at (1,0), resulting in the grid:
 * [[1,0,0,0],
 *  [0,0,0,0]]
 * Once again, all remaining bricks are still stable, so no bricks fall.
 * Hence the result is [0,0].
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 200
 * 	grid[i][j] is 0 or 1.
 * 	1 <= hits.length <= 4 * 10^4
 * 	hits[i].length == 2
 * 	0 <= xi <= m - 1
 * 	0 <= yi <= n - 1
 * 	All (xi, yi) are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bricks-falling-when-hit/
// discuss: https://leetcode.com/problems/bricks-falling-when-hit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {

    pub fn find_with_compression(parents: &mut HashMap<i32, i32>, target : i32) -> i32 {
        if parents.contains_key(&target) {
            let mut root = target;
            while root != parents[&root] {
                root = parents[&root];
            }

            let mut target = target;
            while target != parents[&target] {
                let tmp = parents[&target];
                parents.insert(target, root);
                target = tmp;
            }
            root
        } else {
            -1
        }
    }

    pub fn union(parents: &mut HashMap<i32, i32>, subset_sizes: &mut HashMap<i32, usize>, e1 : i32, e2 : i32, col_count : i32) -> bool {
        let mut e1_root : i32 = Self::find_with_compression(parents, e1);
        if e1_root == -1 {
            parents.insert(e1, e1);
            subset_sizes.insert(e1, 1usize);
            e1_root = e1;
        }

        let mut e2_root : i32 = Self::find_with_compression(parents, e2);
        if e2_root == -1 {
            parents.insert(e2, e2);
            subset_sizes.insert(e2, 1usize);
            e2_root = e2;
        }

        // println!("parents={:?}, subset_sizes={:?}, e1={}, e1_root={}, e2={}, e2_root={}", parents, subset_sizes, e1, e1_root, e2, e2_root);

        if e1_root == e2_root { return false; } 

        let mut by_size = false;
        let mut from_root : i32 = 0;
        let mut from_size = 0usize;
        let mut to_root : i32 = 0;
        let mut to_size = 0usize;

        if e1_root / col_count == 0 && e2_root / col_count == 0 {
            by_size = true;
        } else if e1_root / col_count == 0 {
            to_root = e1_root;
            from_root = e2_root;
        } else if e2_root / col_count == 0 {
            to_root = e2_root;
            from_root = e1_root;
        } else {
            by_size = true;
        }

        if by_size {
            let e1_root_size = subset_sizes[&e1_root];
            let e2_root_size = subset_sizes[&e2_root];
            if  e1_root_size < e2_root_size {
                from_size = e1_root_size;
                from_root = e1_root;
                to_root = e2_root;
                to_size = e2_root_size;
            } else {
                from_size = e2_root_size;
                from_root = e2_root;
                to_root = e1_root;
                to_size = e1_root_size;
            }
        }

        parents.insert(from_root, to_root);
        subset_sizes.insert(to_root, to_size + from_size);
        true
    }



    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> { 
        // build post-hit grid
        // union all bricks, with bricks at top are preferred as representatives. 
        // count and mark all unstable bricks. 
        // for each hit reversely. 
        //   union the neighbor bricks. 
        //   count unstable bricks that turn stable. 
        let mut post_grid = grid.clone();
        for hit in &hits {
            post_grid[hit[0] as usize][hit[1] as usize] = 0;
        }

        let row_count : usize = grid.len();
        let col_count : usize = grid[0].len();

        let mut parents : HashMap<i32, i32> = HashMap::new();
        let mut subset_sizes : HashMap<i32, usize> = HashMap::new();

        for i in 0..row_count {
            for j in 0..col_count {
                let e1 = i * col_count + j;
                if post_grid[i][j] == 1 {
                    Self::union(&mut parents, &mut subset_sizes, e1 as i32, e1 as i32, col_count as i32);
                }

                if post_grid[i][j] == 1 && i < row_count-1 && post_grid[i+1][j] == 1 {
                    let e1 = i * col_count + j;
                    let e2 = (i+1) * col_count + j;
                    Self::union(&mut parents, &mut subset_sizes, e1 as i32, e2 as i32, col_count as i32);
                }

                if post_grid[i][j] == 1 && j < col_count-1 && post_grid[i][j+1] == 1 {
                    let e1 = i * col_count + j;
                    let e2 = i * col_count + j + 1;
                    Self::union(&mut parents, &mut subset_sizes, e1 as i32, e2 as i32, col_count as i32);
                }
            }
        }


        // println!("POST parents:{:?}, subset_sizes:{:?}", parents, subset_sizes);

        let mut unstables : HashSet<i32> = HashSet::new();
        for (e, parent) in &parents {
            if parent/ (col_count as i32) != 0 {
                unstables.insert(*e);
            }
        }
        let mut result = vec![];
        for i in (0..hits.len()).rev() {
            let hit_i = hits[i][0] as usize;
            let hit_j = hits[i][1] as usize;
            if grid[hit_i][hit_j] == 0 {
                result.push(0);
                continue;
            }
            let e : i32 = (hit_i * col_count + hit_j) as i32;
            post_grid[hit_i][hit_j] = 1;

            Self::union(&mut parents, &mut subset_sizes, e, e, col_count as i32);

            if 0 < hit_i && post_grid[hit_i-1][hit_j] == 1 {
                let e1 : i32 = ((hit_i-1) * col_count + hit_j) as i32;
                Self::union(&mut parents, &mut subset_sizes, e, e1, col_count as i32);
            }

            if 0 < hit_j && post_grid[hit_i][hit_j-1] == 1 {
                let e1 : i32 = (hit_i * col_count + hit_j-1) as i32;
                Self::union(&mut parents, &mut subset_sizes, e, e1, col_count as i32);
            }

            if hit_i + 1 < row_count && post_grid[hit_i+1][hit_j] == 1 {
                let e1 : i32 = ((hit_i+1) * col_count + hit_j) as i32;
                Self::union(&mut parents, &mut subset_sizes, e, e1, col_count as i32);
            }

            if hit_j + 1 < col_count && post_grid[hit_i][hit_j+1] == 1 {
                let e1 : i32 = (hit_i * col_count + hit_j+1) as i32;
                Self::union(&mut parents, &mut subset_sizes, e, e1, col_count as i32);
            }

            let this_root : i32 = Self::find_with_compression(&mut parents, e) as i32;
            if this_root / (col_count as i32) != 0 {
                unstables.insert(e);
            }
            // println!("AFTER hit={:?} parents:{:?}, subset_sizes:{:?}, unstables={:?}", hits[i], parents, subset_sizes, unstables);

            let mut stables = vec![];
            for &unstable in &unstables {
                let root : i32 = Self::find_with_compression(&mut parents, unstable);
                // println!("\tunstable={}, root={}", unstable, root);
                if root / (col_count as i32) == 0 {
                    stables.push(unstable);
                }
            }

            for &stable in &stables {
                unstables.remove(&stable);
            }
            result.push(stables.len() as i32);
            // println!("\tunstables={:?}, stables={:?}, result={:?}", unstables, stables, result);

        }
        result.reverse();
        result
    }
    
    pub fn dfs(grid: &mut Vec<Vec<i32>>, i : i32, j : i32, set_size: &mut usize) {
        let row_count = grid.len() as i32;
        let col_count = grid[0].len() as i32;
        if 0 <= i && i < row_count && 0 <= j && j < col_count && grid[i as usize][j as usize] == -1 {
            grid[i as usize][j as usize] = 1;
            *set_size+=1;
            Self::dfs(grid, i+1,j, set_size);
            Self::dfs(grid, i-1,j, set_size);
            Self::dfs(grid, i,j+1, set_size);
            Self::dfs(grid, i,j-1, set_size);
        }
    }


    pub fn hit_bricks_dfs(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        for hit in &hits {
            let i = hit[0] as usize;
            let j = hit[1] as usize;
            if grid[i][j] != 1 {
                result.push(0);
                continue;
            }
            grid[i][j] = 0;
            let row_count : usize = grid.len();
            let col_count : usize = grid[0].len();

            let mut cur_blk_count : usize = 0;
            for i in 0..row_count {
                for j in 0..col_count {
                    if grid[i][j] == 1 {
                        grid[i][j] = -1;
                        cur_blk_count +=1;
                    }
                }
            }

            for j in 0..col_count {
                if grid[0][j] == -1 {
                    let mut set_size : usize = 0;
                    // set brick to 1 during traversal.
                    Self::dfs(&mut grid, 0, j as i32, &mut set_size);
                    cur_blk_count -= set_size;
                }
            }
            result.push(cur_blk_count as i32);
        }

        // for each hit.
        //   if hits a brick, mark the position as 0. 
        //   mark all bricks as -1 and count as cur_brick. 
        //   for -1/brick at the top,
        //     perform dfs to mark neighbor as i and return set_size. 
        //     cur_brick -= set size
        //   push result with cur_brick, which is the fallen bricks.  
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_803() {
        assert_eq!(Solution::hit_bricks(vec![vec![1,0,0,0],vec![1,1,1,0]],  vec![vec![1,0]]), vec![2]);

        assert_eq!(Solution::hit_bricks(vec![vec![1,0,0,0],vec![1,1,0,0]], vec![vec![1,1], vec![1,0]]), vec![0,0]);

        assert_eq!(Solution::hit_bricks(vec![vec![1],vec![1],vec![1],vec![1],vec![1]], vec![vec![3,0], vec![4,0], vec![1,0], vec![2,0], vec![0,0]]), vec![1,0,1,0,0]);


        assert_eq!(Solution::hit_bricks(vec![vec![1,1,1],vec![0,1,0],vec![0,0,0]], vec![vec![0,2],vec![2,0],vec![0,1],vec![1,2]]), vec![0,0,1,0]);

    }
}
