// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.42: Ben's once-serialized
//! account against the text's per-call account. Both wrap the same one
//! serializer around the same operations -- Ben's only builds the
//! protected procedures at account creation instead of at every message
//! -- so what concurrency is allowed is identical. The run deposits
//! through both versions from two threads at once; every unit survives
//! in both, which is the serialization the question is about.

use std::sync::Arc;

use ch03::sec_3_1::Reply;
use ch03::sec_3_4::{
    Serializer, SharedInt, parallel_execute, read_shared, shared_int, shared_withdraw,
};

/// How many concurrent deposits each version must absorb.
const DEPOSITS: usize = 1000;

/// Ben's account: the exercise's `ben_make_account`. The serialized
/// procedures are built once, when the account is made, over one
/// serializer; every later message reuses them.
struct BenAccount {
    balance: SharedInt,
    protected_withdraw: Box<dyn Fn(i128) -> Reply + Send + Sync>,
    protected_deposit: Box<dyn Fn(i128) -> Reply + Send + Sync>,
}

/// The exercise's changed `make_account`: the `protect` calls moved
/// outside the dispatch, so the closures close over the serializer one
/// time instead of once per message.
fn ben_make_account(initial: i128) -> BenAccount {
    let balance = shared_int(initial);
    let protected = Serializer::new();
    let protected_withdraw = {
        let balance = Arc::clone(&balance);
        let protected = protected.clone();
        Box::new(move |amount: i128| protected.protect(|| shared_withdraw(&balance, amount)))
    };
    let protected_deposit = {
        let balance = Arc::clone(&balance);
        let protected = protected.clone();
        Box::new(move |amount: i128| {
            protected.protect(|| {
                let new_balance = read_shared(&balance) + amount;
                ch03::sec_3_4::write_shared(&balance, new_balance);
                Reply::Balance(new_balance)
            })
        })
    };
    BenAccount {
        balance,
        protected_withdraw,
        protected_deposit,
    }
}

/// Deposits one unit into Ben's account, `count` times, half of the
/// deposits on a spawned thread and half on the calling thread, and
/// then withdraws 25 through the once-built withdrawal. Both of the
/// account's stored procedures share the one serializer, which is what
/// the exercise is about; the withdrawal lands after the threads join,
/// so the final balance is exact.
#[must_use]
fn concurrent_ben_deposits(account: BenAccount, count: usize) -> i128 {
    let BenAccount {
        balance,
        protected_withdraw,
        protected_deposit,
    } = account;
    let deposit = Arc::new(protected_deposit);
    let deposit2 = Arc::clone(&deposit);
    let half = count / 2;
    let _ = parallel_execute(
        move |_run| {
            for _ in 0..half {
                let _ = deposit(1);
            }
        },
        move |_run| {
            for _ in 0..(count - half) {
                let _ = deposit2(1);
            }
        },
    );
    let _ = protected_withdraw(25);
    read_shared(&balance)
}

mod ex_3_42 {
    use super::{DEPOSITS, ben_make_account, concurrent_ben_deposits};

    use ch03::sec_3_4::{Account, concurrent_deposits};

    /// Exercise 3.42: serialize once outside dispatch
    ///
    /// Runs the same `count` concurrent deposits and the same final
    /// 25-unit withdrawal through Ben's once-serialized account and
    /// through the text's per-call account, and answers both final
    /// balances.
    #[must_use]
    pub fn ex_3_42() -> (i128, i128) {
        let ben = concurrent_ben_deposits(ben_make_account(100), DEPOSITS);
        let account = Account::new(100);
        let _ = concurrent_deposits(&account, DEPOSITS);
        let _ = account.withdraw(25);
        let text = account.balance();
        (ben, text)
    }
}

#[test]
fn ex_3_42() {
    let (ben, text) = ex_3_42::ex_3_42();
    // Both versions conserve every one of the thousand deposits and
    // land on the same final balance after the 25-unit withdrawal: the
    // two processes' protected bodies cannot interleave, in either
    // arrangement of the same serializer.
    let deposits = i128::try_from(DEPOSITS).expect("deposit count fits in a balance");
    let expected = 100 + deposits - 25;
    assert_eq!(ben, expected);
    assert_eq!(text, expected);
    // The once-built procedures serialize exactly as the per-call ones
    // did: no schedule separates the two accounts' behavior.
    assert_eq!(ben, text);
}
