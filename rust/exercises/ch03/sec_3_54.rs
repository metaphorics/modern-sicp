// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 3.54 and the edition addition
//! 3.54a: `mul-streams` and the factorial stream built from it, then the
//! Catalan numbers built the same way, cross-checked against the closed
//! form computed from the factorials stream.

mod ex_3_54 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.54`.
        pub exercise: &'static str,
    }

    /// Exercise 3.54: mul-streams and factorial stream
    ///
    /// Answers the first eight elements of `factorials` and the element
    /// at index 10 of the same stream.
    pub fn ex_3_54() -> Result<(Vec<i128>, i128), Pending> {
        Err(Pending { exercise: "3.54" })
    }
}

mod ex_3_54a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.54a`.
        pub exercise: &'static str,
    }

    /// Edition addition 3.54a: stream of Catalan numbers
    ///
    /// Answers the first 21 elements of the self-referential Catalan
    /// stream, the statement's closed form read off the factorials
    /// stream for n = 0..=16 -- as far as i128 factorials reach -- and
    /// the overflow-safe form of the same formula through n = 20. All
    /// three must agree element for element.
    #[expect(
        clippy::type_complexity,
        reason = "the pending report mirrors the solution's answer type"
    )]
    pub fn ex_3_54a() -> Result<(Vec<i128>, Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.54a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_54() {
    let (prefix, at_ten) = ex_3_54::ex_3_54().expect("solved");
    // factorials = 1 cons (integers * factorials): element k is k! (the
    // head 1 serving as both 0! and 1!).
    assert_eq!(prefix, vec![1, 1, 2, 6, 24, 120, 720, 5040]);
    assert_eq!(at_ten, 3_628_800);
}

#[test]
#[ignore = "pending solution"]
fn ex_3_54a() {
    let (from_stream, closed, extended) = ex_3_54a::ex_3_54a().expect("solved");
    // C_0 through C_10: the canonical Catalan numbers.
    assert_eq!(
        from_stream[..11],
        [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796]
    );
    // The recurrence's division is exact at every step: (n + 2) always
    // divides C_n * (4n + 2), and the walk lands on the stream's own
    // element at n = 20.
    let mut step = 1_i128;
    for n in 0_u32..20 {
        let numerator = step * (4 * i128::from(n) + 2);
        assert_eq!(numerator % (i128::from(n) + 2), 0);
        step = numerator / (i128::from(n) + 2);
    }
    assert_eq!(step, from_stream[20]);
    // Stream, factorials-stream closed form, and overflow-safe form of
    // the same formula agree.
    assert_eq!(from_stream[..17], closed[..]);
    assert_eq!(from_stream, extended);
}
