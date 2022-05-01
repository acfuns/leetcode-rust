// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

/// # 所用算法
/// 中序遍历、双指针（归并排序）

pub fn get_all_elements(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut t1: Vec<i32> = Vec::new();
    let mut t2: Vec<i32> = Vec::new();
    let mut res: Vec<i32> = Vec::new();

    dfs(root1, &mut t1);
    dfs(root2, &mut t2);

    let n1 = t1.len();
    let n2 = t2.len();
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < n1 && j < n2 {
        if t1[i] < t2[j] {
            res.push(t1[i]);
            i += 1;
        } else {
            res.push(t2[j]);
            j += 1;
        }
    }
    while i < n1 {
        res.push(t1[i]);
        i += 1;
    }
    while j < n2 {
        res.push(t2[j]);
        j += 1;
    }
    res
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
    if let Some(node) = root {
        dfs(node.borrow_mut().left.take(), arr);
        arr.push(node.borrow().val);
        dfs(node.borrow_mut().right.take(), arr);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1305() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let ans = vec![0, 1, 1, 2, 3, 4];
        assert_eq!(ans, get_all_elements(root1, root2));
    }
}
