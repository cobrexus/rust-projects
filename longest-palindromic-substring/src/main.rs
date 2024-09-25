// Solution to https://leetcode.com/problems/longest-palindromic-substring/description/

fn main() {
    println!("{}", Solution::longest_palindrome("c".to_string()));
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut lower_bound = 0;
        let mut upper_bound = 0;

        for i in 0..s.len() {
            'a: for j in i + 1..s.len() {
                if j - i < upper_bound - lower_bound {
                    continue 'a;
                }

                let mut k = i;
                let mut l = j;

                while k <= l {
                    if s[k] != s[l] {
                        continue 'a;
                    }

                    k += 1;
                    l -= 1;
                }

                lower_bound = i;
                upper_bound = j;
            }
        }

        if upper_bound == 0 {
            (s[0] as char).to_string()
        } else {
            s[lower_bound..=upper_bound]
                .iter()
                .map(|b| *b as char)
                .collect::<String>()
        }
    }
}
