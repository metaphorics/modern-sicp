// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.4: lockout after seven bad
//! passwords.

/// Exercise 3.4: lockout after seven bad passwords
///
/// A guarded account counts consecutive wrong-password attempts and
/// calls the installed `call_the_cops` closure on the attempt that goes
/// past seven; a correct password resets the count.
use std::cell::Cell;
use std::rc::Rc;

use ch03::sec_3_1::Request;

mod ex_3_04 {

    use ch03::sec_3_1::{Account, Reply, Request, make_account};
    use std::cell::Cell;
    use std::rc::Rc;
    pub struct GuardedAccount {
        password: String,
        account: Account,
        consecutive_failures: Cell<u32>,
        call_the_cops: Rc<dyn Fn()>,
    }

    /// Exercise 3.4: lockout after seven bad passwords
    ///
    /// Builds the password-protected account of exercise 3.3 plus the
    /// failure counter and the alarm.
    #[must_use]
    pub fn make_account_guarded(
        balance: i128,
        password: &str,
        call_the_cops: impl Fn() + 'static,
    ) -> GuardedAccount {
        GuardedAccount {
            account: make_account(balance),
            password: password.to_string(),
            consecutive_failures: Cell::new(0),
            call_the_cops: Rc::new(call_the_cops),
        }
    }

    impl GuardedAccount {
        /// Sends a request under `password`, counting the failures.
        pub fn send(&self, password: &str, request: Request) -> Reply {
            if password == self.password {
                self.consecutive_failures.set(0);
                return self.account.send(request);
            }
            let failures = self.consecutive_failures.get() + 1;
            self.consecutive_failures.set(failures);
            if failures > 7 {
                (self.call_the_cops)();
            }
            Reply::Message("Incorrect password")
        }
    }

    /// Exercise 3.4: lockout after seven bad passwords
    ///
    /// Returns whether the cops were called after seven consecutive
    /// wrong-password attempts (they must not be) and after the eighth
    /// (they must be).
    #[must_use]
    pub fn ex_3_04() -> (bool, bool) {
        let cops = Rc::new(Cell::new(0_u32));
        let alarm = Rc::clone(&cops);
        let acc = make_account_guarded(100, "secret-password", move || {
            alarm.set(alarm.get() + 1);
        });
        for _ in 0..7 {
            acc.send("wrong", Request::Deposit(1));
        }
        let called_by_seventh = cops.get() > 0;
        acc.send("wrong", Request::Deposit(1));
        let called_by_eighth = cops.get() > 0;
        (called_by_seventh, called_by_eighth)
    }
}

#[test]
fn ex_3_04() {
    assert_eq!(ex_3_04::ex_3_04(), (false, true));

    // A correct password resets the count: seven failures, one success,
    // seven more failures still leaves the cops uncalled, and only the
    // next failure trips the alarm.
    let cops = Rc::new(Cell::new(0_u32));
    let alarm = Rc::clone(&cops);
    let acc = ex_3_04::make_account_guarded(100, "pw", move || {
        alarm.set(alarm.get() + 1);
    });
    for _ in 0..7 {
        acc.send("bad", Request::Deposit(0));
    }
    acc.send("pw", Request::Deposit(0));
    for _ in 0..7 {
        acc.send("bad", Request::Deposit(0));
    }
    assert_eq!(cops.get(), 0);
    acc.send("bad", Request::Deposit(0));
    assert_eq!(cops.get(), 1);
}
