// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.1.1

//! Section 3.1.1: local state variables — the first `withdraw`, the
//! encapsulated `new-withdraw`, the factory `make-withdraw`, and the
//! `make-account` message-passing object.

use std::cell::Cell;

use ch03::sec_3_1::{Reply, Request, make_account, make_withdraw, new_withdraw};

fn main() {
    // The first `withdraw` keeps its `balance` in the enclosing scope: a
    // `Cell` the closure borrows. Any later code in this scope could call
    // `balance.set` too, which is exactly the exposure the text warns
    // about before the balance is made internal.
    let balance = Cell::new(100);
    let withdraw = |amount: i128| {
        if balance.get() >= amount {
            balance.set(balance.get() - amount);
            Reply::Balance(balance.get())
        } else {
            Reply::Message("Insufficient funds")
        }
    };
    let answer = withdraw(25);
    println!("{answer}");
    // => 75
    assert_eq!(answer, Reply::Balance(75));

    let answer = withdraw(25);
    println!("{answer}");
    // => 50
    assert_eq!(answer, Reply::Balance(50));

    let answer = withdraw(60);
    println!("{answer}");
    // => "Insufficient funds"
    assert_eq!(answer, Reply::Message("Insufficient funds"));

    let answer = withdraw(15);
    println!("{answer}");
    // => 35
    assert_eq!(answer, Reply::Balance(35));

    // `new-withdraw`: the balance is now bound inside the block that
    // builds the closure, so no code outside can name it.
    let mut new_withdraw = new_withdraw();
    let answer = new_withdraw(25);
    println!("{answer}");
    // => 75
    assert_eq!(answer, Reply::Balance(75));

    let answer = new_withdraw(25);
    println!("{answer}");
    // => 50
    assert_eq!(answer, Reply::Balance(50));

    // `make-withdraw` builds independent withdrawal processors: each
    // call captures its own formal parameter.
    let mut w1 = make_withdraw(100);
    let mut w2 = make_withdraw(100);

    let answer = w1(50);
    println!("{answer}");
    // => 50
    assert_eq!(answer, Reply::Balance(50));

    let answer = w2(70);
    println!("{answer}");
    // => 30
    assert_eq!(answer, Reply::Balance(30));

    let answer = w2(40);
    println!("{answer}");
    // => "Insufficient funds"
    assert_eq!(answer, Reply::Message("Insufficient funds"));

    let answer = w1(40);
    println!("{answer}");
    // => 10
    assert_eq!(answer, Reply::Balance(10));

    // `make-account` handles deposits as well as withdrawals, with the
    // request as the message. `acc2` is a completely separate object.
    let acc = make_account(100);
    let answer = acc.send(Request::Withdraw(50));
    println!("{answer}");
    // => 50
    assert_eq!(answer, Reply::Balance(50));

    let answer = acc.send(Request::Withdraw(60));
    println!("{answer}");
    // => "Insufficient funds"
    assert_eq!(answer, Reply::Message("Insufficient funds"));

    let answer = acc.send(Request::Deposit(40));
    println!("{answer}");
    // => 90
    assert_eq!(answer, Reply::Balance(90));

    let answer = acc.send(Request::Withdraw(60));
    println!("{answer}");
    // => 30
    assert_eq!(answer, Reply::Balance(30));

    let acc2 = make_account(100);
    let answer = acc2.send(Request::Withdraw(60));
    println!("{answer}");
    // => 40
    assert_eq!(answer, Reply::Balance(40));

    // A deposit of zero reads the balance without changing it: `acc`
    // still holds the 30 that `acc2`'s withdrawal never touched.
    let answer = acc.send(Request::Deposit(0));
    println!("{answer}");
    // => 30
    assert_eq!(answer, Reply::Balance(30));
}
