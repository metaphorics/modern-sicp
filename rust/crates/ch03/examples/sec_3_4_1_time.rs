// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.4.1

//! Section 3.4.1: the nature of time in concurrent systems -- the shared
//! balance that two processes reach, the lost update of Figure 3.29 run
//! for real, and the halt handle of `parallel-execute`.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use ch03::sec_3_1::{Reply, make_withdraw};
use ch03::sec_3_4::{SharedInt, parallel_execute, read_shared, shared_int, shared_withdraw};

/// Withdraws through the shared cell the way the two banking machines of
/// the section do: one lock per access.
fn run_banking_machines() -> i128 {
    let balance = shared_int(100);
    let balance2: SharedInt = Arc::clone(&balance);
    let _ = parallel_execute(
        |_run| shared_withdraw(&balance, 10),
        |_run| shared_withdraw(&balance2, 25),
    );
    read_shared(&balance)
}

fn main() {
    // Time enters through assignment: the same expression, evaluated
    // twice, answers two different values.
    let mut withdraw = make_withdraw(100);
    let reply = withdraw(25);
    println!("{reply}");
    // => 75
    assert_eq!(reply, Reply::Balance(75));

    let reply = withdraw(25);
    println!("{reply}");
    // => 50
    assert_eq!(reply, Reply::Balance(50));

    // Two banking machines, one balance, no ordering imposed: the final
    // balance is 65 when Paul's assignment lands second, and 75 when the
    // interleaving of Figure 3.29 loses Peter's update.
    let observed = run_banking_machines();
    println!("{observed}");
    // => 65 or 75, one value per schedule
    assert!(
        observed == 65 || observed == 75,
        "impossible balance {observed}"
    );

    // The control object of the book's footnote: halt sets one shared
    // flag, the worker polls it between its own steps, and stops early.
    let budget = 1_000_000;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::clone(&counter);
    let (final_count, _halt_request, handle) = parallel_execute(
        move |run| {
            while !run.halted() && counter.load(Ordering::Acquire) < budget {
                counter.fetch_add(1, Ordering::AcqRel);
                std::thread::yield_now();
            }
            counter.load(Ordering::Acquire)
        },
        move |run| {
            while counter2.load(Ordering::Acquire) < 1 {
                std::thread::yield_now();
            }
            run.halt();
        },
    );
    println!("{}", handle.halted());
    // => true
    assert!(handle.halted());
    // The counter stopped because of the flag, not because it ran out.
    println!("{final_count}");
    // => a number below the budget
    assert!(final_count < budget);
}
