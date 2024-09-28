// Solution to https://leetcode.com/problems/longest-palindromic-substring/description/

fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut lower_bound = 0;
        let mut upper_bound = 0;

        let mut all_same = true;

        for i in 1..s.len() {
            if s[i] != s[0] {
                all_same = false;
            }
        }

        if all_same {
            return s.iter().map(|b| *b as char).collect::<String>();
        }

        for i in 0..s.len() - 1 {
            // odd palindrome

            let mut l = i;
            let mut r = i;

            while s[l] == s[r] {
                if r - l > upper_bound - lower_bound {
                    upper_bound = r;
                    lower_bound = l;
                }

                if l == 0 || r == s.len() - 1 {
                    break;
                }

                l -= 1;
                r += 1;
            }

            // even palindrome

            let mut l = i;
            let mut r = i + 1;

            while s[l] == s[r] {
                if r - l > upper_bound - lower_bound {
                    upper_bound = r;
                    lower_bound = l;
                }

                if l == 0 || r == s.len() - 1 {
                    break;
                }

                l -= 1;
                r += 1;
            }
        }

        s[lower_bound..=upper_bound]
            .iter()
            .map(|b| *b as char)
            .collect::<String>()
    }
}
