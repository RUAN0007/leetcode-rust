use std::collections::{HashMap, VecDeque};

/**
 * [126] Word Ladder II
 *
 * A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
 * 
 * 	Every adjacent pair of words differs by a single letter.
 * 	Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
 * 	sk == endWord
 * 
 * Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest transformation sequences from beginWord to endWord, or an empty list if no such sequence exists. Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].
 *  
 * Example 1:
 * 
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
 * Output: [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
 * Explanation: There are 2 shortest transformation sequences:
 * "hit" -> "hot" -> "dot" -> "dog" -> "cog"
 * "hit" -> "hot" -> "lot" -> "log" -> "cog"
 * 
 * Example 2:
 * 
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
 * Output: []
 * Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= beginWord.length <= 10
 * 	endWord.length == beginWord.length
 * 	1 <= wordList.length <= 5000
 * 	wordList[i].length == beginWord.length
 * 	beginWord, endWord, and wordList[i] consist of lowercase English letters.
 * 	beginWord != endWord
 * 	All the words in wordList are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-ladder-ii/
// discuss: https://leetcode.com/problems/word-ladder-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// The current traversed path in vector, 
// The index of the flipped char in map. 
type Path = Vec<String>; 
impl Solution {
    pub fn diff_by_one(word_a : &String, word_b : &String) -> Option<usize> {
        let mut diff = None;
        for (i, char_a) in word_a.chars().enumerate() {
            if char_a != word_b.chars().nth(i).unwrap() {
                if let None = diff {
                    diff = Some(i);
                } else {
                    return None; // diff more than 1. 
                }
            }
        }
        diff
    }
    // TODO: timeout 
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {

        let mut paths : VecDeque<Path> = VecDeque::new();
        let mut result = vec![];
        let mut least_length = usize::MAX;
        paths.push_back(vec![begin_word]);
        while let Some(path) = paths.pop_front() {
            let (mut seq) = path;
            println!("{:?}", seq);
            if least_length < seq.len() {
                // traversed to a longer path than the known, 
                // stop here as later paths are all longer. 
                break
            }
            let cur_word = seq.last().unwrap().clone();
            if cur_word == end_word {
                // spot a valid path
                least_length = seq.len();
                result.push(seq);
            } else {
                // find all words that differ cur_word by 1 char.
                //   but the differed pos is NOT in visited
                // Enqueue them. 
                for word in &word_list {
                    // let diff_pos = Self::diff_by_one(&cur_word, &word);
                    // println!("cur_word: {}, word: {}, diff_pos: {:?}", cur_word, word, diff_pos);

                    if let Some(diff_pos) = Self::diff_by_one(&cur_word, &word) {
                        if !seq.iter().any(|seq_word| *seq_word == *word) {
                            let mut new_seq = seq.clone();
                            new_seq.push(word.clone());
                            paths.push_back(new_seq);
                        }
                    }
                }
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
    fn test_126() {
        assert_eq!(
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                vec_string!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            vec![
                vec_string!["hit", "hot", "dot", "dog", "cog"],
                vec_string!["hit", "hot", "lot", "log", "cog"],
            ]
        );
        assert_eq!(
            Solution::find_ladders(
                "cet".to_owned(),
                "ism".to_owned(),
                vec_string![
                    "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip",
                    "kay", "per", "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die",
                    "war", "hay", "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan", "cot",
                    "bid", "ali", "pay", "col", "gum", "ger", "row", "won", "dan", "rum", "fad",
                    "tut", "sag", "yip", "sui", "ark", "has", "zip", "fez", "own", "ump", "dis",
                    "ads", "max", "jaw", "out", "btu", "ana", "gap", "cry", "led", "abe", "box",
                    "ore", "pig", "fie", "toy", "fat", "cal", "lie", "noh", "sew", "ono", "tam",
                    "flu", "mgm", "ply", "awe", "pry", "tit", "tie", "yet", "too", "tax", "jim",
                    "san", "pan", "map", "ski", "ova", "wed", "non", "wac", "nut", "why", "bye",
                    "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log", "tod", "dot",
                    "bow", "fob", "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib", "mel",
                    "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm", "lot", "tom",
                    "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit",
                    "doe", "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron",
                    "soy", "gin", "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop",
                    "but", "orb", "fen", "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod",
                    "yam", "pew", "web", "hod", "hun", "gyp", "wei", "wis", "rob", "gad", "pie",
                    "mon", "dog", "bib", "rub", "ere", "dig", "era", "cat", "fox", "bee", "mod",
                    "day", "apr", "vie", "nev", "jam", "pam", "new", "aye", "ani", "and", "ibm",
                    "yap", "can", "pyx", "tar", "kin", "fog", "hum", "pip", "cup", "dye", "lyx",
                    "jog", "nun", "par", "wan", "fey", "bus", "oak", "bad", "ats", "set", "qom",
                    "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao", "mom", "mas",
                    "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may", "shy",
                    "rid", "bat", "sum", "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava",
                    "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim", "sec", "ltd",
                    "you", "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw",
                    "nix", "ate", "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any",
                    "dow", "cod", "bed", "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao",
                    "aug", "mum", "hoe", "bah", "hal", "keg", "hew", "zed", "tow", "gog", "ass",
                    "dem", "who", "bet", "gos", "son", "ear", "spy", "kit", "boy", "due", "sen",
                    "oaf", "mix", "hep", "fur", "ada", "bin", "nil", "mia", "ewe", "hit", "fix",
                    "sad", "rib", "eye", "hop", "haw", "wax", "mid", "tad", "ken", "wad", "rye",
                    "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin", "mad", "ray", "hon",
                    "roy", "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay", "poi", "yep",
                    "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee", "wig",
                    "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit",
                    "hem", "joy", "bum", "rio", "yon", "dec", "leg", "put", "sue", "dim", "pet",
                    "yaw", "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc", "moe", "caw",
                    "eel", "dix", "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt",
                    "lid", "pun", "ton", "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil",
                    "jig", "hub", "low", "did", "tin", "get", "gte", "sox", "lei", "mig", "fig",
                    "lon", "use", "ban", "flo", "nov", "jut", "bag", "mir", "sty", "lap", "two",
                    "ins", "con", "ant", "net", "tux", "ode", "stu", "mug", "cad", "nap", "gun",
                    "fop", "tot", "sow", "sal", "sic", "ted", "wot", "del", "imp", "cob", "way",
                    "ann", "tan", "mci", "job", "wet", "ism", "err", "him", "all", "pad", "hah",
                    "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com", "ill", "was",
                    "cab", "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran", "lab",
                    "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac",
                    "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig",
                    "cid", "god", "duo", "lin", "aid", "gel", "awl", "lag", "elf", "liz", "ref",
                    "aha", "fib", "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned", "coo",
                    "win", "tao", "coy", "van", "man", "pit", "guy", "foe", "hid", "mai", "sup",
                    "jay", "hob", "mow", "jot", "are", "pol", "arc", "lax", "aft", "alb", "len",
                    "air", "pug", "pox", "vow", "got", "meg", "zoe", "amp", "ale", "bud", "gee",
                    "pin", "dun", "pat", "ten", "mob"
                ]
            ),
            vec![
                vec_string![
                    "cet", "get", "gee", "gte", "ate", "ats", "its", "ito", "ibo", "ibm", "ism"
                ],
                vec_string![
                    "cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"
                ],
                vec_string![
                    "cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"
                ],
            ]
        );
    }
}
