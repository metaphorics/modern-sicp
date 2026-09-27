// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.38: the balances Peter, Paul, and
//! Mary can leave, run for real on threads.

mod ex_3_38 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.38`.
        pub exercise: &'static str,
    }

    /// Exercise 3.38: enumerate interleaved balance outcomes
    ///
    /// Answers the sorted balances of the six sequential orders and the
    /// sorted distinct balances of unforced concurrent runs.
    pub fn ex_3_38() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.38" })
    }
}

mod ex_3_38a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.38a`.
        pub exercise: &'static str,
    }

    /// Exercise 3.38a: run interleavings with real threads
    ///
    /// Answers one forced schedule per sequential order and per lost
    /// update, each paired with the balance its forced run left.
    pub fn ex_3_38a() -> Result<Vec<(&'static str, i128)>, Pending> {
        Err(Pending { exercise: "3.38a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_38() {
    let Ok((sequential, concurrent)) = ex_3_38::ex_3_38() else {
        panic!("ex_3_38 scaffold reports pending");
    };
    assert_eq!(sequential, vec![35, 40, 45, 50]);
    assert!(!concurrent.is_empty());
}

#[test]
#[ignore = "pending solution"]
fn ex_3_38a() {
    let Ok(forced) = ex_3_38a::ex_3_38a() else {
        panic!("ex_3_38a scaffold reports pending");
    };
    assert_eq!(forced.len(), 9);
    assert_eq!(forced[0], ("peter,paul,mary", 45));
}
