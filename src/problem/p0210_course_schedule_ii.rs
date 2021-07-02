/**
 * [210] Course Schedule II
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
 * 
 * 	For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
 * 
 * Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.
 *  
 * Example 1:
 * 
 * Input: numCourses = 2, prerequisites = [[1,0]]
 * Output: [0,1]
 * Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
 * 
 * Example 2:
 * 
 * Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
 * Output: [0,2,1,3]
 * Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
 * So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
 * 
 * Example 3:
 * 
 * Input: numCourses = 1, prerequisites = []
 * Output: [0]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= numCourses <= 2000
 * 	0 <= prerequisites.length <= numCourses * (numCourses - 1)
 * 	prerequisites[i].length == 2
 * 	0 <= ai, bi < numCourses
 * 	ai != bi
 * 	All the pairs [ai, bi] are distinct.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule-ii/
// discuss: https://leetcode.com/problems/course-schedule-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pred : HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut succ : HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..(num_courses as usize) {
            pred.insert(i, HashSet::new());
            succ.insert(i, HashSet::new());
        }


        for prerequisite in prerequisites.iter() {
            let s : usize = prerequisite[0] as usize;
            let p : usize = prerequisite[1] as usize;
            pred.get_mut(&s).unwrap().insert(p);
            succ.get_mut(&p).unwrap().insert(s);
        }

        // key: # of predecessor, value: course id with that # of predecessor
        let mut pred_counts : BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
        for (&cid, preds) in pred.iter() {
            pred_counts.entry(preds.len()).or_insert(HashSet::new()).insert(cid);
        }
        // println!("pred={:?}", pred);
        // println!("succ={:?}\n", succ);
        // println!("pred_counts={:?}", pred_counts);

        let mut result : Vec<i32> = vec![];
        while pred_counts.len() > 0 {
            // println!("pred_counts: {:?}", pred_counts);
            let (&smallest_pred_count, cids) = pred_counts.iter_mut().next().unwrap();
            // println!("smallest_pred_count: {:?}", smallest_pred_count);
            if smallest_pred_count != 0 { return vec![]; }
            let cid : usize = *cids.iter().next().unwrap();
            cids.remove(&cid);
            if cids.len() == 0 {
                pred_counts.remove(&smallest_pred_count);
            }

            result.push(cid as i32);
            
            // println!("cid={}", cid);
            for &succ_cid in succ[&cid].iter() {
                // println!("\tsucc_cid={}", succ_cid);
                let cur_pred_count : usize = pred[&succ_cid].len();
                pred_counts.get_mut(&cur_pred_count).unwrap().remove(&succ_cid);
                if pred_counts[&cur_pred_count].len() == 0 {
                    pred_counts.remove(&cur_pred_count);
                }
                pred_counts.entry(cur_pred_count - 1).or_insert(HashSet::new()).insert(succ_cid);
                pred.get_mut(&succ_cid).unwrap().remove(&cid);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_210() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
    }
}
