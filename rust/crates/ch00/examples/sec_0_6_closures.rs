// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.6, listing: a closure factory, then heterogeneous closures
//! behind `dyn Fn`.

use ch00::sec_0_6::{apply_all, make_multiplier};

fn main() {
    let triple = make_multiplier(3);
    let result = triple(7);
    println!("{result}");
    // => 21
    assert_eq!(result, 21);

    let ops: Vec<Box<dyn Fn(i64) -> i64>> =
        vec![Box::new(make_multiplier(2)), Box::new(|x| x + 100)];
    let results = apply_all(&ops, 5);
    println!("{results:?}");
    // => [10, 105]
    assert_eq!(results, vec![10, 105]);
}
