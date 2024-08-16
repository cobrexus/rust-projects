// Solution to https://leetcode.com/problems/string-to-integer-atoi/description/

fn main() {}

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim().chars().collect::<Vec<_>>();
        let mut output = String::new();

        let nums_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        if s.len() == 0 {
            return 0;
        }

        if s[0] == '-' {
            output.push('-');
            let _ = s.remove(0);
        } else if s[0] == '+' {
            let _ = s.remove(0);
        }

        if s.len() == 0 {
            return 0;
        }

        while s.len() > 0 && s[0] == '0' {
            let _ = s.remove(0);
        }

        if s.len() == 0 {
            return 0;
        }

        while s.len() > 0 && nums_chars.contains(&s[0]) {
            output.push(s[0]);
            s.remove(0);
        }

        match output.parse::<i128>() {
            Ok(x) => {
                if x > i32::MAX as i128 {
                    return i32::MAX;
                } else if x < i32::MIN as i128 {
                    return i32::MIN;
                } else {
                    return x as i32;
                }
            }
            Err(_) => {
                let mut is_valid_int = true;
                for c in s {
                    if !nums_chars.contains(&c) {
                        is_valid_int = false;
                    }
                }
                if is_valid_int {
                    return i32::MAX;
                } else {
                    return 0;
                }
            }
        }
    }
}
