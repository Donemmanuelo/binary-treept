pub struct TreeNode {
    pub value: i32, 
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
pub fn preorder_traversal(root: Option<&TreeNode>) {
    if let Some(node) = root {
        println!("{}", node.value);
        preorder_traversal(node.left.as_deref());
        preorder_traversal(node.right.as_deref());
    }
}