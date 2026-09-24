// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.3: a password-protected
//! account.

/// A password-protected bank account: the balance object of the
/// section, wrapped with the password it was created under.
use ch03::sec_3_1::{Reply, Request};

mod ex_3_03 {

    use ch03::sec_3_1::{Account, Reply, Request, make_account};
    pub struct PasswordAccount {
        password: String,
        account: Account,
    }

    /// Exercise 3.3: a password-protected account
    ///
    /// The account answers a request only when the accompanying password
    /// matches the one it was created with.
    #[must_use]
    pub fn make_account_with_password(balance: i128, password: &str) -> PasswordAccount {
        PasswordAccount {
            password: password.to_string(),
            account: make_account(balance),
        }
    }

    impl PasswordAccount {
        /// Sends a request guarded by `password`; a mismatch draws the
        /// complaint and leaves the balance alone.
        pub fn send(&self, password: &str, request: Request) -> Reply {
            if password == self.password {
                self.account.send(request)
            } else {
                Reply::Message("Incorrect password")
            }
        }
    }

    fn balance_of(reply: Reply) -> i128 {
        match reply {
            Reply::Balance(n) => n,
            Reply::Message(m) => unreachable!("expected a balance, got {m:?}"),
        }
    }

    /// Exercise 3.3: a password-protected account
    ///
    /// Returns the balance answered to a correct-password withdrawal of 40
    /// from a 100 account, and the complaint text answered to a
    /// wrong-password deposit.
    #[must_use]
    pub fn ex_3_03() -> (i128, String) {
        let acc = make_account_with_password(100, "secret-password");
        let good = acc.send("secret-password", Request::Withdraw(40));
        let bad = acc.send("some-other-password", Request::Deposit(50));
        let complaint = match bad {
            Reply::Message(m) => m,
            Reply::Balance(n) => unreachable!("a rejected request answers with a message, got {n}"),
        };
        (balance_of(good), complaint.to_string())
    }
}

#[test]
fn ex_3_03() {
    assert_eq!(ex_3_03::ex_3_03(), (60, "Incorrect password".to_string()));

    // A wrong password leaves the balance untouched: the rejected
    // deposit of 50 must not have landed.
    let acc = ex_3_03::make_account_with_password(100, "open-sesame");
    let rejected = acc.send("wrong", Request::Deposit(50));
    assert_eq!(
        acc.send("open-sesame", Request::Deposit(0)),
        Reply::Balance(100)
    );
    assert_eq!(rejected, Reply::Message("Incorrect password"));
}
