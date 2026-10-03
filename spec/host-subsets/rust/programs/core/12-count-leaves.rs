// SPDX-License-Identifier: GPL-3.0-only
// Case: core/12-count-leaves. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Counting leaves over a recursive tree: the Box indirection makes
/// the recursion well typed and the count structural.
enum Tree {
    Leaf(i64),
    Branch(Box<Tree>, Box<Tree>),
}

fn count_leaves(tree: &Tree) -> i64 {
    match tree {
        Tree::Leaf(_) => 1,
        Tree::Branch(left, right) => count_leaves(left) + count_leaves(right),
    }
}

fn main() {
    let tree = Tree::Branch(
        Box::new(Tree::Leaf(1)),
        Box::new(Tree::Branch(Box::new(Tree::Leaf(2)), Box::new(Tree::Leaf(3)))),
    );
    println!("{}", count_leaves(&tree));
}
