struct BinaryTreeNode<E> {
    value: E,
    left: Option<*mut BinaryTreeNode<E>>,
    right: Option<*mut BinaryTreeNode<E>>,
}

struct BinaryTree<E> {
    root: *mut BinaryTreeNode<E>,
}
