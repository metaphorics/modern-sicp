// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.52: what `accum`'s shared sum cell
//! reads after EACH expression of the book's sequence, run with the
//! memoized delay, and after the same `z` walk re-run over a fresh
//! `seq` that re-executes `accum` for every element.

mod ex_3_52 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.52`.
        pub exercise: &'static str,
    }

    /// Exercise 3.52: accum traces assignment plus laziness
    ///
    /// Answers the memoized walk of the book's sequence -- the sum after
    /// each definition, `stream-ref y 7`, the full `z`, and the final
    /// sum -- then the counterfactual: `z` rebuilt over a fresh `seq`
    /// from the same sum cell, which re-runs `accum` from wherever the
    /// cell stands.
    #[expect(
        clippy::type_complexity,
        reason = "the pending report mirrors the solution's answer type"
    )]
    pub fn ex_3_52() -> Result<(Vec<i128>, i128, Vec<i128>, i128, Vec<i128>, i128), Pending> {
        Err(Pending { exercise: "3.52" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_52() {
    let (build_sums, at_y_seven, z_values, sum_final, fresh_z, fresh_sum) =
        ex_3_52::ex_3_52().expect("solved");
    // The delay holds accum back: defining seq computes only its head
    // (1), defining y forces seq to its first even element (6), and
    // defining z forces seq on to its first multiple of 5 (10).
    assert_eq!(build_sums, vec![1, 6, 10]);
    // The 8th even partial sum is 136, which is also the running sum:
    // reaching y index 7 computes seq through 1 + ... + 16.
    assert_eq!(at_y_seven, 136);
    // The full z walk scans seq to its end, so every multiple of 5 in
    // the 20-element seq comes out, and the running sum lands at
    // 210 = 1 + ... + 20.
    assert_eq!(z_values, vec![10, 15, 45, 55, 105, 120, 190, 210]);
    assert_eq!(sum_final, 210);
    // Not memoized: the fresh seq re-executes accum starting from 210,
    // so every element shifts up by 210 and the same walk lands at 420.
    assert_eq!(fresh_z, vec![220, 225, 255, 265, 315, 330, 400, 420]);
    assert_eq!(fresh_sum, 420);
}
