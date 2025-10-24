use nary_tree::NodeRef;

pub fn tree_any<F, N>(node: &NodeRef<N>, f: &F) -> bool
where
    F: Fn(&N) -> bool,
{
    if f(node.data()) {
        return true;
    }
    for child in node.children() {
        if tree_any(&child, f) {
            return true;
        }
    }
    false
}
