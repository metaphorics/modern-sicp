// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.48: the deadlock-avoidance
//! technique for the exchange problem. Each account gets a unique
//! number at creation, and a serialized exchange always enters the
//! lower-numbered account's serializer first, so two processes
//! exchanging the same two accounts from opposite sides queue in the
//! same order instead of crossing. Two real threads trade 200 opposite
//! -direction exchanges over the $100 and $50 accounts; every one
//! completes -- a deadlock would strand a thread and the scope would
//! never join -- and the bank still holds its $150.

use ch03::sec_3_4::{AccountAndSerializer, exchange};

/// How many exchanges each of the two opposite directions runs.
const EXCHANGES_PER_DIRECTION: usize = 100;

/// An account with the unique number the ordering technique needs.
struct OrderedAccount {
    number: usize,
    account: AccountAndSerializer,
}

impl OrderedAccount {
    fn new(number: usize, initial: i128) -> Self {
        Self {
            number,
            account: AccountAndSerializer::new(initial),
        }
    }

    /// The book's rewritten `serialized-exchange`: whichever way the
    /// caller names the accounts, the process always enters the
    /// lower-numbered account's serializer first, then the higher's,
    /// and only then runs the exchange body. The body swaps the two
    /// balances whole, since the difference moves from one account
    /// entirely into the other.
    fn exchange_with(&self, other: &OrderedAccount) -> i128 {
        let (first, second) = if self.number <= other.number {
            (self, other)
        } else {
            (other, self)
        };
        let outer = first.account.serializer();
        let inner = second.account.serializer();
        outer.protect(|| inner.protect(|| exchange(&first.account, &second.account)))
    }
}

/// Runs `count` opposite-direction ordered exchanges between the two
/// accounts on two real threads, and answers the sorted balances with
/// the number of exchanges that completed. A deadlock would leave a
/// thread stuck and the scope would never join.
#[must_use]
fn ordered_trade(count: usize) -> (Vec<i128>, usize) {
    let a = OrderedAccount::new(1, 100);
    let b = OrderedAccount::new(2, 50);
    let done = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..count {
                a.exchange_with(&b);
                done.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });
        scope.spawn(|| {
            for _ in 0..count {
                b.exchange_with(&a);
                done.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });
    });
    let exchanges = done.load(std::sync::atomic::Ordering::SeqCst);
    let mut balances = vec![a.account.balance(), b.account.balance()];
    balances.sort_unstable();
    (balances, exchanges)
}

mod ex_3_48 {
    use super::{EXCHANGES_PER_DIRECTION, ordered_trade};

    /// Exercise 3.48: deadlock avoidance by lock ordering
    ///
    /// Answers the sorted balances after the opposite-order ordered
    /// exchanges and how many exchanges completed.
    #[must_use]
    pub fn ex_3_48() -> (Vec<i128>, usize) {
        ordered_trade(EXCHANGES_PER_DIRECTION)
    }
}

#[test]
fn ex_3_48() {
    let (balances, exchanges) = ex_3_48::ex_3_48();
    // Two hundred opposite-direction exchanges, every one completed:
    // with both processes entering account 1's serializer first there
    // is no crossing to deadlock in.
    assert_eq!(exchanges, 2 * EXCHANGES_PER_DIRECTION);
    // Each completed exchange swaps the two balances exactly once --
    // the whole body ran under both serializers, so no interleaving
    // could lose or invent a dollar -- and the bank's $150 are still
    // all there, in the same two accounts.
    assert_eq!(balances.iter().sum::<i128>(), 150);
    assert_eq!(balances.len(), 2);
}
