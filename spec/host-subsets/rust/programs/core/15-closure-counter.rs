// SPDX-License-Identifier: GPL-3.0-only
// Case: core/15-closure-counter. Provenance: derived from teaching
// execution; parent confirms natively. Never native-proven.
//
// The lesson: lexical mutable capture. `make_counter` returns an
// `FnMut` closure that owns its state through `move` and mutates it on
// each call; the two counters are independent frames.

/// Builds one counter closure.
fn make_counter() -> Box<dyn FnMut() -> i64 + 'static> {
    let mut value = 0;
    Box::new(move || {
        value += 1;
        value
    })
}

fn main() {
    let mut first = make_counter();
    let mut second = make_counter();
    println!("{}", first());
    println!("{}", first());
    println!("{}", second());
}
