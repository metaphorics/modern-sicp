// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.7: a joint account shares one
//! balance.

use ch03::sec_3_1::{Reply, Request};

mod ex_3_07 {
    use ch03::sec_3_1::{Account, Reply, Request, make_account};

    /// The password-protected account of exercise 3.3, repeated here so
    /// each solution file stands alone.
    pub struct PasswordAccount {
        password: String,
        account: Account,
    }

    /// Builds the password-protected account of exercise 3.3.
    #[must_use]
    pub fn make_account_with_password(balance: i128, password: &str) -> PasswordAccount {
        PasswordAccount {
            password: password.to_string(),
            account: make_account(balance),
        }
    }

    impl PasswordAccount {
        /// Sends a request guarded by `password`.
        pub fn send(&self, password: &str, request: Request) -> Reply {
            if password == self.password {
                self.account.send(request)
            } else {
                Reply::Message("Incorrect password")
            }
        }

        /// Checks `password` and hands out another name for the shared
        /// balance on success, or the complaint on failure.
        ///
        /// # Errors
        /// The account's complaint when the password does not match.
        pub fn authorize(&self, password: &str) -> Result<Account, Reply> {
            if password == self.password {
                Ok(self.account.clone())
            } else {
                Err(Reply::Message("Incorrect password"))
            }
        }

        /// Wraps `account` under a new password.
        #[must_use]
        pub fn joint(account: Account, password: &str) -> PasswordAccount {
            PasswordAccount {
                password: password.to_string(),
                account,
            }
        }
    }

    /// Exercise 3.7: a joint account shares one balance
    ///
    /// Verifies `password` against the existing account, then opens a
    /// second password front-end over the *same* balance object. The
    /// sharing is what forces the balance to live behind a reference
    /// counter: two closures cannot each own their own copy.
    ///
    /// # Errors
    /// Answers the account's complaint when `password` does not match.
    pub fn make_joint(
        account: &PasswordAccount,
        password: &str,
        new_password: &str,
    ) -> Result<PasswordAccount, Reply> {
        let shared = account.authorize(password)?;
        Ok(PasswordAccount::joint(shared, new_password))
    }

    fn balance_of(reply: Reply) -> i128 {
        match reply {
            Reply::Balance(n) => n,
            Reply::Message(m) => unreachable!("expected a balance, got {m:?}"),
        }
    }

    /// Exercise 3.7: a joint account shares one balance
    ///
    /// Returns the balance a joint-account withdrawal of 40 from a 100
    /// account answers, the balance the original account then reports, and
    /// whether a joint opened with the wrong password is refused.
    #[must_use]
    pub fn ex_3_07() -> (i128, i128, bool) {
        let peter_acc = make_account_with_password(100, "open-sesame");
        let paul_acc = make_joint(&peter_acc, "open-sesame", "rosebud")
            .expect("the right password opens a joint account");
        let joint_withdrawal = paul_acc.send("rosebud", Request::Withdraw(40));
        let peter_view = peter_acc.send("open-sesame", Request::Deposit(0));
        let refused = make_joint(&peter_acc, "wrong-password", "rosebud").is_err();
        (
            balance_of(joint_withdrawal),
            balance_of(peter_view),
            refused,
        )
    }
}

#[test]
fn ex_3_07() {
    assert_eq!(ex_3_07::ex_3_07(), (60, 60, true));

    // The joint front-end shares the one balance: a withdrawal through
    // the new password is visible through the old one.
    let peter = ex_3_07::make_account_with_password(100, "open-sesame");
    let paul = ex_3_07::make_joint(&peter, "open-sesame", "rosebud")
        .expect("the right password opens a joint account");
    assert_eq!(
        paul.send("rosebud", Request::Withdraw(70)),
        Reply::Balance(30)
    );
    assert_eq!(
        peter.send("open-sesame", Request::Deposit(0)),
        Reply::Balance(30)
    );
}
