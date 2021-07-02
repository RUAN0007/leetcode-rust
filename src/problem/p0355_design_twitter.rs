/**
 * [355] Design Twitter
 *
 * Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and is able to see the 10 most recent tweets in the user's news feed.
 * Implement the Twitter class:
 * 
 * 	Twitter() Initializes your twitter object.
 * 	void postTweet(int userId, int tweetId) Composes a new tweet with ID tweetId by the user userId. Each call to this function will be made with a unique tweetId.
 * 	List<Integer> getNewsFeed(int userId) Retrieves the 10 most recent tweet IDs in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user themself. Tweets must be ordered from most recent to least recent.
 * 	void follow(int followerId, int followeeId) The user with ID followerId started following the user with ID followeeId.
 * 	void unfollow(int followerId, int followeeId) The user with ID followerId started unfollowing the user with ID followeeId.
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
 * [[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
 * Output
 * [null, null, [5], null, null, [6, 5], null, [5]]
 * Explanation
 * Twitter twitter = new Twitter();
 * twitter.postTweet(1, 5); // User 1 posts a new tweet (id = 5).
 * twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
 * twitter.follow(1, 2);    // User 1 follows user 2.
 * twitter.postTweet(2, 6); // User 2 posts a new tweet (id = 6).
 * twitter.getNewsFeed(1);  // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
 * twitter.unfollow(1, 2);  // User 1 unfollows user 2.
 * twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= userId, followerId, followeeId <= 500
 * 	0 <= tweetId <= 10^4
 * 	All the tweets have unique IDs.
 * 	At most 3 * 10^4 calls will be made to postTweet, getNewsFeed, follow, and unfollow.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-twitter/
// discuss: https://leetcode.com/problems/design-twitter/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

struct Twitter {
    follows : HashMap<i32, HashSet<i32>>,
    tweets : HashMap<i32, VecDeque<(usize, i32)>>,
    tweet_counter : usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{follows: HashMap::new(), tweets: HashMap::new(), tweet_counter: 0}     
    }
    
    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.follows.entry(user_id).or_insert([user_id].iter().cloned().collect());        

        self.tweets.entry(user_id).or_insert(VecDeque::new()).push_back((self.tweet_counter, tweet_id));
        self.tweet_counter+=1;
        if self.tweets[&user_id].len() > 10 {
            self.tweets.get_mut(&user_id).unwrap().pop_front();
        }
    }
    
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        if self.follows.get(&user_id).is_none() {return vec![];}

        let mut result : Vec<i32> = vec![];
        // (tweet counter, user_id, user_tweet_idx)
        let mut recent_tweets : BinaryHeap<(usize, i32, usize)> = BinaryHeap::new();
        for &followee_id in self.follows.get(&user_id).unwrap().iter() {
            if let Some(tweet_infos) = self.tweets.get(&followee_id) {
                let last_tweet_idx : usize = tweet_infos.len() - 1;
                let last_tweet_counter : usize = tweet_infos[last_tweet_idx].0;
                recent_tweets.push((last_tweet_counter, followee_id, last_tweet_idx));
            }
        }

        while result.len() < 10 {
            if let Some((_, user_id, mut user_tweet_idx)) = recent_tweets.pop() {
                result.push(self.tweets[&user_id][user_tweet_idx].1);
                if user_tweet_idx > 0 {
                    user_tweet_idx -= 1;
                    let user_tweet_counter : usize = self.tweets[&user_id][user_tweet_idx].0;
                    recent_tweets.push((user_tweet_counter, user_id, user_tweet_idx));
                }
            } else {
                break;
            }
        }
        result
    }
    
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_insert([follower_id].iter().cloned().collect()).insert(followee_id);
    }
    
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.follows.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_355() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5); // User 1 posts a new tweet (id = 5).
        assert_eq!(twitter.get_news_feed(1), vec![5]);  // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
        twitter.follow(1, 2);    // User 1 follows user 2.
        twitter.post_tweet(2, 6); // User 2 posts a new tweet (id = 6).
        assert_eq!(twitter.get_news_feed(1), vec![6,5]);  // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
        twitter.unfollow(1, 2);  // User 1 unfollows user 2.
        assert_eq!(twitter.get_news_feed(1), vec![5]);  // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
    }
}
