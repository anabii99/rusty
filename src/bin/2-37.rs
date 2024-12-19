// Definition for singly-linked list.
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

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut carry = 0;

        let mut l1 = l1;
        let mut l2 = l2;

        while l1 != None || l2 != None || carry != 0 {
            let val1 = match l1 {
                Some(ref node) => node.val,
                None => 0,
            };
            let val2 = match l2 {
                Some(ref node) => node.val,
                None => 0,
            };

            let v = val1 + val2 + carry;
            carry = v / 10;
            set(&mut ret, v % 10);

            if let Some(node) = l1 {
                l1 = node.next;
            }
            if let Some(node) = l2 {
                l2 = node.next;
            }
        }

        ret
    }
}

fn set(node: &mut Option<Box<ListNode>>, val: i32) {
    match node {
        Some(v) => {
            set(&mut v.next, val);
        }
        None => {
            *node = Some(Box::new(ListNode { val, next: None }));
        }
    }
}

// Helper functions for testing
fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        vec.push(node.val);
        current = node.next;
    }
    vec
}

struct Solution;
fn main() {
    let l1 = to_linked_list(vec![2, 4, 3]);
    let l2 = to_linked_list(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(result)); // Output should be [7, 0, 8]
}
