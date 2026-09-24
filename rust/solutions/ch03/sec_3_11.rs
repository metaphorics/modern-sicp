// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.11: where account state lives,
//! answered with identity experiments and with the model frames built
//! as real nodes.

use ch03::sec_3_1::Reply;

mod ex_3_11 {

    use std::rc::Rc;

    use ch03::sec_3_1::{Reply, Request, make_account};
    use ch03::sec_3_2::{define, int_of, make_withdraw_procedure};
    use sicp_runtime::{Env, Value};

    /// Exercise 3.11: where account state lives
    ///
    /// Runs the book's interaction on one account and probes what the
    /// drawing would show. Returns `(alias is the same object, second
    /// account is the same object, balance seen through the original,
    /// model frames distinct, model frames share the global frame)`.
    #[must_use]
    pub fn ex_3_11() -> (bool, bool, Reply, bool, bool) {
        // Two names, one account: a clone of an `Account` is a second
        // name for the same state cell.
        let acc = make_account(50);
        let alias = acc.clone();
        let alias_same = acc.same_object_as(&alias);

        // A second factory call: same code, a different cell.
        let acc2 = make_account(100);
        let acc2_same = acc.same_object_as(&acc2);

        // The book's interaction, run through the alias name.
        let _ = alias.send(Request::Deposit(40));
        let _ = alias.send(Request::Withdraw(60));

        // Reading through the original name sees what the alias did.
        let balance_through_acc = acc.send(Request::Deposit(0));

        // The same picture for the model frames: E1 and E2 hold the two
        // balances, and a procedure object over E1 reaches only E1.
        let global = Env::global();
        let e1 = Env::child(&global);
        define(&e1, "balance", Value::int(50));
        let e2 = Env::child(&global);
        define(&e2, "balance", Value::int(100));
        let frames_distinct = !Rc::ptr_eq(&e1, &e2);
        let frames_share_global = frames_extend(&e1, &global) && frames_extend(&e2, &global);
        let mut withdraw_e1 = make_withdraw_procedure(Rc::clone(&e1));
        let mut withdraw_e2 = make_withdraw_procedure(Rc::clone(&e2));
        let _ = withdraw_e1(60);
        let _ = withdraw_e2(60);
        assert_eq!(int_of(&e1, "balance"), Ok(50));
        assert_eq!(int_of(&e2, "balance"), Ok(40));

        (
            alias_same,
            acc2_same,
            balance_through_acc,
            frames_distinct,
            frames_share_global,
        )
    }

    /// Whether `env` extends `global` directly, by pointer identity.
    fn frames_extend(env: &Rc<Env>, global: &Rc<Env>) -> bool {
        env.outer
            .as_ref()
            .is_some_and(|outer| Rc::ptr_eq(outer, global))
    }
}

#[test]
fn ex_3_11() {
    let (alias_same, acc2_same, balance, frames_distinct, frames_share_global) = ex_3_11::ex_3_11();

    // The alias and the original are one object: 50 + 40 - 60 = 30
    // through either name.
    assert!(alias_same);
    assert_eq!(balance, Reply::Balance(30));

    // The second account is a different object: nothing about it is
    // reachable from acc, though both carry the same compiled send.
    assert!(!acc2_same);

    // In the model frames: distinct frames per factory call, both
    // extending the same global frame, and each procedure object
    // mutating exactly its own frame.
    assert!(frames_distinct);
    assert!(frames_share_global);
}
