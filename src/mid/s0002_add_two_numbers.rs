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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut dummy = None;
    let mut p = &mut dummy;
    let mut t = 0;
    while l1 != None || l2 != None || t != 0 {
        if let Some(j1) = l1 {
            t += j1.val;
            l1 = j1.next;
        }
        if let Some(j2) = l2 {
            t += j2.val;
            l2 = j2.next;
        }
        // 对应dummy可变
        *p = Some(Box::new(ListNode::new(t % 10)));
        t /= 10;
        // 对应p引用可变
        // as_mut() 表示box没有实现copy特征，所以为了不让unwrap把所有权转移，就要用as_mut()方法。
        p = &mut p.as_mut().unwrap().next;
    }
    dummy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0002() {
        let t3 = ListNode::new(3);

        let t4 = ListNode {
            val: 4,
            next: Some(Box::new(t3)),
        };

        let t2 = ListNode {
            val: 2,
            next: Some(Box::new(t4)),
        };

        let b4 = ListNode::new(4);

        let b6 = ListNode {
            val: 6,
            next: Some(Box::new(b4)),
        };

        let b5 = ListNode {
            val: 5,
            next: Some(Box::new(b6)),
        };
        assert_eq!(
            add_two_numbers(Some(Box::new(t2)), Some(Box::new(b5))),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        );
    }
}
