// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.12: the assembler's
//! instruction-use summary over the section's two recursive machines.

use ch05::sec_5_2::{factorial_machine, fibonacci_machine};
use sicp_runtime::Value;

mod ex_5_12 {
    //! Exercise 5.12: extend the assembler to collect the
    //! instruction types, the entry-point registers, the stacked
    //! registers, and each register's assign sources, then examine
    //! the lists for the Fibonacci machine of Figure 5.12.

    use super::*;

    /// The Fibonacci machine's summary. Eighteen distinct
    /// instructions: eight assigns, three saves, three restores, two
    /// gotos, one test, one branch. `continue` is the only
    /// entry-point register and the only register besides `n` and
    /// `val` that the stack sees. The sources of `n` are its two
    /// decrements and `val`; the sources of `val` are `n` itself and
    /// the addition; the sources of `continue` are the three labels.
    #[test]
    fn ex_5_12_fibonacci_summary() {
        let machine = fibonacci_machine();
        let summary = machine.instruction_use();
        assert_eq!(
            summary.counts,
            [
                ("assign", 8),
                ("branch", 1),
                ("goto", 2),
                ("restore", 3),
                ("save", 3),
                ("test", 1)
            ]
            .into_iter()
            .map(|(kind, count)| (kind.to_owned(), count))
            .collect()
        );
        assert_eq!(
            summary.entry_points,
            ["continue".to_owned()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        );
        assert_eq!(
            summary.stack_registers,
            ["continue".to_owned(), "n".to_owned(), "val".to_owned()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        );
        let sources: Vec<(String, Vec<String>)> = summary
            .sources
            .iter()
            .map(|(reg, set)| (reg.clone(), set.iter().cloned().collect()))
            .collect();
        assert_eq!(
            sources,
            [
                (
                    "continue".to_owned(),
                    vec![
                        "(label afterfib-n-1)".to_owned(),
                        "(label afterfib-n-2)".to_owned(),
                        "(label fib-done)".to_owned(),
                    ]
                ),
                (
                    "n".to_owned(),
                    vec![
                        "(op - (reg n) (const 1))".to_owned(),
                        "(op - (reg n) (const 2))".to_owned(),
                        "(reg val)".to_owned(),
                    ]
                ),
                (
                    "val".to_owned(),
                    vec!["(op + (reg val) (reg n))".to_owned(), "(reg n)".to_owned(),]
                ),
            ]
        );
    }

    /// The factorial machine's summary, cross-checking the same
    /// machinery: thirteen distinct instructions, and the sources of
    /// `val` are exactly the book's example, `(const 1)` and
    /// `((op *) (reg n) (reg val))`.
    #[test]
    fn ex_5_12_factorial_summary() {
        let machine = factorial_machine();
        let summary = machine.instruction_use();
        let distinct: usize = summary.counts.values().sum();
        assert_eq!(distinct, 13);
        assert_eq!(
            summary.entry_points,
            ["continue".to_owned()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        );
        assert_eq!(
            summary.stack_registers,
            ["continue".to_owned(), "n".to_owned()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        );
        let val_sources = &summary.sources["val"];
        assert_eq!(
            *val_sources,
            [
                "(const 1)".to_owned(),
                "(op * (reg n) (reg val))".to_owned(),
            ]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
        );
        let n_sources = &summary.sources["n"];
        assert_eq!(
            *n_sources,
            ["(op - (reg n) (const 1))".to_owned()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        );
        let continue_sources = &summary.sources["continue"];
        assert_eq!(
            *continue_sources,
            [
                "(label after-fact)".to_owned(),
                "(label fact-done)".to_owned(),
            ]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
        );
        assert_eq!(
            machine.get_register("continue"),
            Ok(Value::sym("*unassigned*"))
        );
    }
}
