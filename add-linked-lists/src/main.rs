// Solution to https://leetcode.com/problems/add-two-numbers/

fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_vec: Vec<i32> = Vec::new();
        let mut l2_vec: Vec<i32> = Vec::new();

        fn to_vec(l: Option<Box<ListNode>>, vec: &mut Vec<i32>) {
            match l {
                None => (),
                Some(b) => {
                    vec.push(b.val);
                    to_vec(b.next, vec);
                }
            }
        }

        to_vec(l1, &mut l1_vec);
        to_vec(l2, &mut l2_vec);

        let mut ans_vec: Vec<i32> = Vec::new();
        let mut idx = 0;
        let mut carry = 0;

        while idx < std::cmp::max(l1_vec.len(), l2_vec.len()) {
            match (l1_vec.get(idx), l2_vec.get(idx)) {
                (None, None) => (),
                (Some(n), None) | (None, Some(n)) => {
                    if *n + carry >= 10 {
                        ans_vec.push(*n + carry - 10);
                        carry = 1;
                    } else {
                        ans_vec.push(*n + carry);
                        carry = 0;
                    }
                }
                (Some(n), Some(o)) => {
                    if *n + *o + carry >= 10 {
                        ans_vec.push(*n + *o + carry - 10);
                        carry = 1;
                    } else {
                        ans_vec.push(*n + *o + carry);
                        carry = 0;
                    }
                }
            }

            idx += 1;
        }

        if carry != 0 {
            ans_vec.push(carry);
        }

        ans_vec = ans_vec.iter().rev().map(|&n| n).collect::<Vec<_>>();

        let mut ans = None::<Box<ListNode>>;

        for n in ans_vec {
            ans = Some(Box::new(ListNode { val: n, next: ans }))
        }

        ans
    }
}
