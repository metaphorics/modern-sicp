// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.3.3

//! Section 2.3.3: three set representations, unordered lists, ordered
//! lists, and binary trees.

use ch02::sec_2_3::{
    Tree, adjoin_set, adjoin_set_tree, element_of_set, element_of_set_ordered, element_of_set_tree,
    intersection_set, intersection_set_ordered,
};

fn main() {
    // Sets as unordered lists.
    let s = [1_i128, 3, 6, 10];
    assert!(element_of_set(6, &s));
    assert!(!element_of_set(7, &s));
    let grown = adjoin_set(7, &s);
    println!("{grown:?}");
    // => [7, 1, 3, 6, 10]
    assert_eq!(grown, vec![7, 1, 3, 6, 10]);
    let same = adjoin_set(6, &s);
    println!("{same:?}");
    // => [1, 3, 6, 10]
    assert_eq!(same, s.to_vec());
    println!("{:?}", intersection_set(&[1, 2, 3], &[2, 3, 4]));
    // => [2, 3]
    assert_eq!(intersection_set(&[1, 2, 3], &[2, 3, 4]), vec![2, 3]);

    // Sets as ordered lists: `1 3 6 10`, increasing.
    let ordered = [1_i128, 3, 6, 10];
    assert!(element_of_set_ordered(6, &ordered));
    assert!(!element_of_set_ordered(7, &ordered));
    println!("{:?}", intersection_set_ordered(&[1, 3, 5, 7], &[3, 5, 9]));
    // => [3, 5]
    assert_eq!(
        intersection_set_ordered(&[1, 3, 5, 7], &[3, 5, 9]),
        vec![3, 5]
    );

    // Sets as binary trees: Figure 2.16's first tree for {1, 3, 5, 7, 9, 11}.
    let tree = Tree::make_tree(
        7,
        Tree::make_tree(
            3,
            Tree::make_tree(1, Tree::Empty, Tree::Empty),
            Tree::make_tree(5, Tree::Empty, Tree::Empty),
        ),
        Tree::make_tree(
            9,
            Tree::Empty,
            Tree::make_tree(11, Tree::Empty, Tree::Empty),
        ),
    );
    println!("{tree}");
    // => (7 (3 (1 () ()) (5 () ())) (9 () (11 () ())))
    assert_eq!(
        tree.to_string(),
        "(7 (3 (1 () ()) (5 () ())) (9 () (11 () ())))"
    );
    assert!(element_of_set_tree(5, &tree));
    assert!(!element_of_set_tree(4, &tree));

    // Adjoining 1 through 7 in sequence to an empty tree produces the
    // highly unbalanced tree of Figure 2.17.
    let mut skewed = Tree::Empty;
    for x in 1..=7 {
        skewed = adjoin_set_tree(x, &skewed);
    }
    println!("{skewed}");
    // => (1 () (2 () (3 () (4 () (5 () (6 () (7 () ())))))))
    assert_eq!(
        skewed.to_string(),
        "(1 () (2 () (3 () (4 () (5 () (6 () (7 () ())))))))"
    );
}
