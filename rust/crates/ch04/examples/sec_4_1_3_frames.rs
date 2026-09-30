// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.3: evaluator data structures. Frames bind names to
//! values, a procedure value carries its parameters and the
//! environment of definition, and the environment operations resolve a
//! name through the chain of frames from the newest outwards.

use ch04::sec_4_1::run_source;
use sicp_runtime::host::hir::BindId;
use sicp_runtime::host::value::{Addr, HostValue, Store, Trap, render_display};

const CLOSURES: &str = "\
fn main() {
    let base = 10;
    let add_ten = |x: i64| x + base;
    let add_thirty = |x: i64| x + 20 + base;
    println!(\"{}\", add_ten(4));
    println!(\"{}\", add_thirty(4));
}
";

fn main() {
    // A new frame over the global one, the arena's `push_frame`, with
    // `x` and `y` written into its slots.
    let mut store = Store::default();
    let global = store.push_frame(0, 2);
    let frame = store.push_frame(10, 2);
    store
        .write(
            Addr {
                frame,
                bind: BindId(10),
            },
            HostValue::Int(3),
        )
        .expect("writes x");
    store
        .write(
            Addr {
                frame,
                bind: BindId(11),
            },
            HostValue::Int(4),
        )
        .expect("writes y");

    // Lookup resolves the newest frame first: `x` is the inner
    // binding, whatever the outer frames hold.
    println!(
        "{}",
        render_display(&store.read(addr(frame, 10)).expect("bound"))
    );
    // => 3
    assert_eq!(store.read(addr(frame, 10)), Ok(HostValue::Int(3)));

    // Assignment rebinds the nearest binding in place; a binding in a
    // newer frame hides the older one without touching it.
    store
        .write(addr(frame, 10), HostValue::Int(30))
        .expect("rebinds x");
    store
        .write(addr(global, 1), HostValue::Int(40))
        .expect("defines y in the global frame");
    println!(
        "{}",
        render_display(&store.read(addr(frame, 10)).expect("bound"))
    );
    // => 30
    assert_eq!(store.read(addr(frame, 10)), Ok(HostValue::Int(30)));
    println!(
        "{}",
        render_display(&store.read(addr(frame, 11)).expect("bound"))
    );
    // => 4
    assert_eq!(store.read(addr(frame, 11)), Ok(HostValue::Int(4)));

    // Moving a value out of a slot leaves the move marker: reading it
    // again answers the trap, never a stale copy.
    assert_eq!(store.take(addr(global, 1)), Ok(HostValue::Int(40)));
    println!("{:?}", store.read(addr(global, 1)).expect_err("moved"));
    // => UseAfterMove
    assert_eq!(store.read(addr(global, 1)).err(), Some(Trap::UseAfterMove));

    // A procedure value carries its parameters and the environment of
    // definition: two closures over one captured binding read that
    // same environment, while their own bodies differ.
    let outcome = run_source(CLOSURES).expect("admitted");
    println!("{}", outcome.stdout);
    // => 14
    // => 34
    assert_eq!(outcome.stdout, "14\n34\n");
}

/// One binding's address in the arena.
fn addr(frame: usize, bind: u32) -> Addr {
    Addr {
        frame,
        bind: BindId(bind),
    }
}
