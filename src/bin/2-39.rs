#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Flatten linked lists into vectors
        let mut nums1 = Vec::new();
        let mut nums2 = Vec::new();

        while let Some(node) = l1 {
            nums1.push(node.val);
            l1 = node.next;
        }
        while let Some(node) = l2 {
            nums2.push(node.val);
            l2 = node.next;
        }

        // Align digits and perform addition
        let mut carry = 0;
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() || j < nums2.len() || carry > 0 {
            let x = if i < nums1.len() { nums1[i] } else { 0 };
            let y = if j < nums2.len() { nums2[j] } else { 0 };

            let sum = x + y + carry;
            result.push(sum % 10);
            carry = sum / 10;

            i += 1;
            j += 1;
        }

        // Build the resulting linked list from the result vector
        let mut head = None;
        for &val in result.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = head;
            head = Some(new_node);
        }

        head
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
