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

        while l1.is_some() || l2.is_some() || carry != 0 {
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

            l1 = if let Some(node) = l1 { node.next } else { None };
            l2 = if let Some(node) = l2 { node.next } else { None };
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

fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = head;
        head = Some(new_node);
    }
    head
}

struct Solution;
fn main() {
    // Example usage:
    let l1 = create_list(vec![2, 4, 3]);
    let l2 = create_list(vec![5, 6, 4]);

    let result = Solution::add_two_numbers(l1, l2);

    // Print the result
    let mut current = result;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
}
