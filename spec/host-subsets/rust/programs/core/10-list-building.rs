// SPDX-License-Identifier: GPL-3.0-only
// Case: core/10-list-building. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Pair structure through admitted indirection: the list is explicit
/// recursive data, which is the pair-structure lesson in typed form.
enum List {
    Cons(i64, Box<List>),
    Nil,
}

fn build(n: i64) -> List {
    let mut result = List::Nil;
    let mut count = 1;
    while count <= n {
        result = List::Cons(count, Box::new(result));
        count += 1;
    }
    result
}

fn length(list: &List) -> i64 {
    match list {
        List::Cons(_, rest) => 1 + length(rest),
        List::Nil => 0,
    }
}

fn main() {
    let list = build(5);
    println!("{}", length(&list));
    println!("{}", length(&build(10)));
}
