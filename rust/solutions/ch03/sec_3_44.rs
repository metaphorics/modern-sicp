// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.44: transferring a named amount
//! from one account into another. The amount arrives with the request,
//! so the procedure reads only one account's balance at a time: the
//! withdrawal serializes on the giving account, the deposit on the
//! receiving account, and no joint serializer is needed. Three accounts
//! starting at $10, $20, and $30 absorb hundreds of concurrent
//! transfers from three directions, and every dollar is still there.

use ch03::sec_3_1::Reply;
use ch03::sec_3_4::AccountAndSerializer;

/// How many transfers each of the three concurrent directions runs.
const TRANSFERS_PER_DIRECTION: usize = 100;

/// Transfers `amount` from one account into another, serializing the
/// withdrawal on the giver's serializer and the deposit on the
/// receiver's. The deposit runs only when the withdrawal succeeded, so
/// an overdrawn attempt moves nothing; there is no moment when the
/// procedure reads both balances, which is the essential difference
/// from `exchange`.
fn transfer(from: &AccountAndSerializer, into: &AccountAndSerializer, amount: i128) -> bool {
    let giver = from.serializer();
    let receiver = into.serializer();
    let withdrawn = giver.protect(|| matches!(from.withdraw(amount), Reply::Balance(_)));
    if !withdrawn {
        return false;
    }
    receiver.protect(|| {
        let _ = into.deposit(amount);
    });
    true
}

/// Runs `count` rounds of three concurrent transfer directions -- $1
/// from `a` to `b`, from `b` to `c`, and from `c` to `a` -- on three
/// real threads, and answers the sorted balances with the total.
#[must_use]
fn concurrent_transfers(count: usize) -> (Vec<i128>, i128) {
    let a = AccountAndSerializer::new(10);
    let b = AccountAndSerializer::new(20);
    let c = AccountAndSerializer::new(30);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..count {
                transfer(&a, &b, 1);
            }
        });
        scope.spawn(|| {
            for _ in 0..count {
                transfer(&b, &c, 1);
            }
        });
        scope.spawn(|| {
            for _ in 0..count {
                transfer(&c, &a, 1);
            }
        });
    });
    let balances = vec![a.balance(), b.balance(), c.balance()];
    let total = balances.iter().sum();
    (balances, total)
}

mod ex_3_44 {
    use super::{TRANSFERS_PER_DIRECTION, concurrent_transfers};

    /// Exercise 3.44: transfer needs no joint lock
    ///
    /// Answers the sorted balances after the concurrent transfers and
    /// their conserved total.
    #[must_use]
    pub fn ex_3_44() -> (Vec<i128>, i128) {
        let (balances, total) = concurrent_transfers(TRANSFERS_PER_DIRECTION);
        let mut sorted = balances;
        sorted.sort_unstable();
        (sorted, total)
    }
}

#[test]
fn ex_3_44() {
    let (balances, total) = ex_3_44::ex_3_44();
    // No joint lock, no loss: every concurrent transfer either moves
    // one dollar or is refused whole, so the $60 never changes.
    assert_eq!(total, 60);
    // The three balances still hold exactly the money: each account is
    // between zero and the whole bank, and no refused transfer moved a
    // dollar (see the refusal test below).
    assert_eq!(balances.len(), 3);
    assert!(*balances.first().expect("three balances") >= 0);
    assert!(*balances.last().expect("three balances") <= 60);
}

#[test]
fn ex_3_44_refused_transfer_moves_nothing() {
    // An overdrawn transfer is refused whole: the deposit never runs.
    let a = AccountAndSerializer::new(5);
    let b = AccountAndSerializer::new(20);
    assert!(!transfer(&a, &b, 10));
    assert_eq!(a.balance(), 5);
    assert_eq!(b.balance(), 20);
}
