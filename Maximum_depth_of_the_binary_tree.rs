struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Adjust the function to correctly handle the node references
fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            // Use pattern matching to access the inner TreeNode
            let left_depth = match &node.left {
                Some(left) => max_depth(Some(left)),
                None => 0,
            };
            let right_depth = match &node.right {
                Some(right) => max_depth(Some(right)),
                None => 0,
            };
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example binary tree:
    //     3
    //    / \
    //   9  20
    //      / \
    //     15  7
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));

    // Pass root as Option<&Box<TreeNode>> to max_depth
    let depth = max_depth(root.as_ref());
    println!("Maximum depth of the binary tree: {}", depth);
}
