/**
 * [332] Reconstruct Itinerary
 *
 * You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure and the arrival airports of one flight. Reconstruct the itinerary in order and return it.
 * All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK". If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.
 * 
 * 	For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
 * 
 * You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/itinerary1-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
 * Output: ["JFK","MUC","LHR","SFO","SJC"]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/itinerary2-graph.jpg" style="width: 222px; height: 230px;" />
 * Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
 * Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
 * Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= tickets.length <= 300
 * 	tickets[i].length == 2
 * 	fromi.length == 3
 * 	toi.length == 3
 * 	fromi and toi consist of uppercase English letters.
 * 	fromi != toi
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reconstruct-itinerary/
// discuss: https://leetcode.com/problems/reconstruct-itinerary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn helper(tickets : &Vec<Vec<String>>, itinerary : &mut HashMap<String, HashMap<String, usize>>, route : &mut Vec<String>) -> bool {
        let cur_pos : String = route.last().unwrap().clone();
        if route.len() == tickets.len() + 1 {
            return true;
        }
        if let Some(next_destinations) = itinerary.get(&cur_pos) {
            let mut destinations : Vec<String> = next_destinations.iter().filter(|(_, &count)|{count > 0}).map(|(dest, _)|{dest.clone()}).collect();
            destinations.sort();

            // println!("cur_pos={}, destinations={:?}", cur_pos, destinations);

            for dest in destinations.iter() {
                *itinerary.get_mut(&cur_pos).unwrap().get_mut(dest).unwrap() -=1;
                route.push(dest.clone());

                if Self::helper(tickets, itinerary, route) {
                    return true;
                }

                route.pop();
                *itinerary.get_mut(&cur_pos).unwrap().get_mut(dest).unwrap() +=1;
            }

        }
        false
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut itinerary : HashMap<String, HashMap<String, usize>> = HashMap::new();
        for (i, ticket) in tickets.iter().enumerate() {
            *itinerary.entry(ticket[0].clone()).or_insert(HashMap::new()).entry(ticket[1].clone()).or_insert(0)+=1;
        }

        // println!("itinerary={:?}", itinerary);

        let mut route : Vec<String> = vec!["JFK".to_owned()];
        Self::helper(&tickets, &mut itinerary, &mut route);
        route
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_332() {
        assert_eq!(Solution::find_itinerary(vec![vec_string!["MUC","LHR"],vec_string!["JFK","MUC"],vec_string!["SFO","SJC"],vec_string!["LHR","SFO"]]), vec_string!["JFK","MUC","LHR","SFO","SJC"]);


        assert_eq!(Solution::find_itinerary(vec![vec_string!["JFK","SFO"],vec_string!["JFK","ATL"],vec_string!["SFO","ATL"],vec_string!["ATL","JFK"],vec_string!["ATL","SFO"]]), vec_string!["JFK","ATL","JFK","SFO","ATL","SFO"]);

        assert_eq!(Solution::find_itinerary(vec![vec_string!["EZE","AXA"],vec_string!["TIA","ANU"],vec_string!["ANU","JFK"],vec_string!["JFK","ANU"],vec_string!["ANU","EZE"],vec_string!["TIA","ANU"],vec_string!["AXA","TIA"],vec_string!["TIA","JFK"],vec_string!["ANU","TIA"],vec_string!["JFK","TIA"]]), vec_string!["JFK","ANU","EZE","AXA","TIA","ANU","JFK","TIA","ANU","TIA","JFK"]);
    }

}
