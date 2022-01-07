use std::collections::LinkedList;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn push(self, val: i32) -> ListNode {
        ListNode {
            next: Some(Box::new(self)),
            val
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn to_vec(self) -> Vec<i32> {
        let mut result: LinkedList<i32> = LinkedList::new();
        let mut curr: Option<Box<ListNode>> = Some(Box::new(self));
        while curr.is_some() {
            result.push_back(curr.as_ref().unwrap().val);
            curr = curr.unwrap().next;
        }
        result.into_iter().collect()
    }
}

#[allow(dead_code)]
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_node_value(l1, l2, 0)
}

fn add_node_value(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, remaining: i32) -> Option<Box<ListNode>> {
    let (sum, l1) = add_value_and_next(remaining, l1);
    let (sum, l2) = add_value_and_next(sum, l2);
    let mut result = ListNode::new(sum % 10);
    if l1.is_some() || l2.is_some() || sum >= 10 {
        result.next = add_node_value(l1, l2, sum / 10);
    }
    Some(Box::new(result))
}

fn add_value_and_next(val: i32, opt: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
    if let Some(node) = opt {
            (val + node.val, node.next)
    } else {
            (val, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = ListNode::new(3).push(4).push(2);
        let l2 = ListNode::new(4).push(6).push(5);
        let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().to_vec(), vec![7,0,8]);
    }

    #[test]
    fn test_add_two_numbers_zero() {
        let l1 = ListNode::new(0);
        let l2 = ListNode::new(0);
        let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().to_vec(), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_nine() {
        let l1 = ListNode::new(9).push(9).push(9).push(9).push(9).push(9).push(9);
        let l2 = ListNode::new(9).push(9).push(9).push(9);
        let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().to_vec(), vec![8,9,9,9,0,0,0,1]);
    }
}
