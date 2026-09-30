// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.2.2

//! Section 2.2.2: trees, `count-leaves`, and `scale-tree`, the
//! recursion that follows the shape of the data.

use ch02::sec_2_2::{Nest, count_leaves, leaf, length, scale_tree, scale_tree_map, sub};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A list of three items whose first item is itself a list, the
    // shape the statement shows. In this edition the nested
    // structure is a `Nest`, and printing shows the shape.
    let x: Nest<i128> = sub(&[sub(&[leaf(1), leaf(2)]), leaf(3), leaf(4)]);
    println!("{x}");
    // => ((1 2) 3 4)
    assert_eq!(x.to_string(), "((1 2) 3 4)");

    // `length` counts items on the spine; `count-leaves` counts all
    // the way down to the leaves.
    if let Nest::Sub(items) = &x {
        println!("{}", length(items));
        // => 3
        assert_eq!(length(items), 3);
    }
    println!("{}", count_leaves(&x));
    // => 4
    assert_eq!(count_leaves(&x), 4);

    // The same tree twice by value: length sees two spines, count
    // -leaves walks both.
    let doubled: Nest<i128> = sub(&[x.clone(), x.clone()]);
    println!("{doubled}");
    // => (((1 2) 3 4) ((1 2) 3 4))
    assert_eq!(doubled.to_string(), "(((1 2) 3 4) ((1 2) 3 4))");
    if let Nest::Sub(items) = &doubled {
        println!("{}", length(items));
        // => 2
        assert_eq!(length(items), 2);
    }
    println!("{}", count_leaves(&doubled));
    // => 8
    assert_eq!(count_leaves(&doubled), 8);

    // `scale-tree` keeps the shape and multiplies every leaf, in the
    // book's two versions: direct recursion, and map over the sub
    // -trees with recursion only where an element is itself a tree.
    let tree: Nest<i128> = sub(&[
        leaf(1),
        sub(&[leaf(2), sub(&[leaf(3), leaf(4)]), leaf(5)]),
        sub(&[leaf(6), leaf(7)]),
    ]);
    let scaled = scale_tree(&tree, 10)?;
    println!("{scaled}");
    // => (10 (20 (30 40) 50) (60 70))
    let scaled_via_map = scale_tree_map(&tree, 10)?;
    println!("{scaled_via_map}");
    // => (10 (20 (30 40) 50) (60 70))
    assert_eq!(scaled.to_string(), "(10 (20 (30 40) 50) (60 70))");
    assert_eq!(scaled_via_map.to_string(), "(10 (20 (30 40) 50) (60 70))");

    Ok(())
}
