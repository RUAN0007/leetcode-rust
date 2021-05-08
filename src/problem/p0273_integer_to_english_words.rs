/**
 * [273] Integer to English Words
 *
 * Convert a non-negative integer num to its English words representation.
 *  
 * Example 1:
 * Input: num = 123
 * Output: "One Hundred Twenty Three"
 * Example 2:
 * Input: num = 12345
 * Output: "Twelve Thousand Three Hundred Forty Five"
 * Example 3:
 * Input: num = 1234567
 * Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
 * Example 4:
 * Input: num = 1234567891
 * Output: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
 *  
 * Constraints:
 * 
 * 	0 <= num <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-to-english-words/
// discuss: https://leetcode.com/problems/integer-to-english-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {return "Zero".to_owned();}
        let place_vals : Vec<String> = vec![
                                            "".to_owned(),
                                            "Thousand".to_owned(),  
                                            "Million".to_owned(),  
                                            "Billion".to_owned(), 
                                            // "Trillion".to_owned(),
                                            // "Quadrillion".to_owned(),
                                            // "Quintillion".to_owned(),
                                            // "Hextillion".to_owned(),
                                            // "Septillion".to_owned(),
                                            // "Octillion".to_owned(),
                                            // "Nonillion".to_owned(),
                                            // "Decillion".to_owned(),
                                            ];
        
        let single_digits : Vec<String> = vec!["".to_owned(), "One".to_owned(), "Two".to_owned(), "Three".to_owned(), 
                                       "Four".to_owned(), "Five".to_owned(), "Six".to_owned(), "Seven".to_owned(),
                                       "Eight".to_owned(), "Nine".to_owned()];

        let ten_pls : Vec<String> = vec!["Ten".to_owned(), "Eleven".to_owned(), "Twelve".to_owned(), "Thirteen".to_owned(), 
                                       "Fourteen".to_owned(), "Fifteen".to_owned(), "Sixteen".to_owned(), "Seventeen".to_owned(),
                                       "Eighteen".to_owned(), "Nineteen".to_owned()];

        let double_digits: Vec<String> = vec!["".to_owned(), "".to_owned(), "Twenty".to_owned(), "Thirty".to_owned(), 
                                       "Forty".to_owned(), "Fifty".to_owned(), "Sixty".to_owned(), "Seventy".to_owned(),
                                       "Eighty".to_owned(), "Ninety".to_owned()];
        let mut words : Vec<String> = vec![];
        for i in (0..(place_vals.len())).rev() {
            let digit_part : usize = ((num / i32::pow(10, 3 * i as u32)) % 1000) as usize;
            if digit_part == 0 {continue}
            let digit2  = digit_part / 100 ;
            let digit1  = digit_part / 10 % 10;
            let digit0  = digit_part % 10 ;

            if digit2 !=0 {
                words.push(single_digits[digit2].clone());
                words.push("Hundred".to_owned());
            }
            if digit1 == 0 {
                if digit0 != 0 {
                    words.push(single_digits[digit0].clone());
                }
            } else if digit1 == 1 {
                words.push(ten_pls[digit0].clone());
            } else {
                words.push(double_digits[digit1].clone());
                if digit0 != 0 {
                    words.push(single_digits[digit0].clone());
                }
            }
            if i > 0 {
                words.push(place_vals[i].clone());
            }
        }
        words.join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_273() {
        assert_eq!(Solution::number_to_words(1234567891), "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_owned());

        assert_eq!(Solution::number_to_words(1234567), "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_owned());

        assert_eq!(Solution::number_to_words(12345), "Twelve Thousand Three Hundred Forty Five".to_owned());
        assert_eq!(Solution::number_to_words(50868), "Fifty Thousand Eight Hundred Sixty Eight".to_owned());
    }
}
