// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.19: breakpoints stop the
//! machine before the nth instruction after a label; the held
//! machine can be examined, proceeded, and its breakpoints cancelled.

use ch05::sec_5_2::{Fault, Stop, gcd_machine};
use sicp_runtime::Value;

fn gcd_with_inputs(a: i128, b: i128) -> ch05::sec_5_2::Machine {
    let mut machine = gcd_machine();
    machine.set_register("a", Value::Int(a)).unwrap();
    machine.set_register("b", Value::Int(b)).unwrap();
    machine
}

mod ex_5_19 {
    //! Exercise 5.19: Alyssa's breakpoints, with proceed and
    //! cancel, over gcd(12, 8).

    use super::*;

    /// A breakpoint at `test-b` 4 sits just before the assignment
    /// `a <- b` (offset 4: the test, the branch, and the assign to
    /// `t` come first). Each stop reports the label and offset and
    /// holds the machine; two holds later the third pass branches
    /// straight to `gcd-done` and the run ends with a = 4.
    #[test]
    fn ex_5_19_breakpoint_holds_and_proceeds() {
        let mut machine = gcd_with_inputs(12, 8);
        machine.set_breakpoint("test-b", 4).unwrap();
        let stop = machine.start().unwrap();
        assert_eq!(
            stop,
            Stop::Breakpoint {
                label: "test-b".to_owned(),
                offset: 4
            }
        );
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(12));
        assert_eq!(machine.get_register("b").unwrap(), Value::Int(8));

        let stop = machine.proceed().unwrap();
        assert_eq!(
            stop,
            Stop::Breakpoint {
                label: "test-b".to_owned(),
                offset: 4
            }
        );
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(8));
        assert_eq!(machine.get_register("b").unwrap(), Value::Int(4));

        assert_eq!(machine.proceed().unwrap(), Stop::End);
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(4));
        assert_eq!(
            machine.transcript(),
            [
                "breakpoint at test-b: 4".to_owned(),
                "breakpoint at test-b: 4".to_owned(),
            ]
        );
    }

    /// Cancelling one breakpoint lets the next start run through;
    /// cancelling all breakpoints clears any hold.
    #[test]
    fn ex_5_19_cancel_breakpoint() {
        let mut machine = gcd_with_inputs(12, 8);
        machine.set_breakpoint("test-b", 4).unwrap();
        machine.cancel_breakpoint("test-b", 4).unwrap();
        assert_eq!(machine.start().unwrap(), Stop::End);
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(4));
        assert!(machine.transcript().is_empty());

        let mut machine = gcd_with_inputs(12, 8);
        machine.set_breakpoint("test-b", 1).unwrap();
        machine.set_breakpoint("test-b", 4).unwrap();
        machine.cancel_all_breakpoints();
        assert_eq!(machine.start().unwrap(), Stop::End);
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(4));
    }

    /// A breakpoint names an instruction: offset zero and offsets
    /// past the sequence are refused, as is an unknown label, and a
    /// set breakpoint is idempotent to re-set.
    #[test]
    fn ex_5_19_bad_breakpoints_refused() {
        let mut machine = gcd_with_inputs(12, 8);
        assert_eq!(
            machine.set_breakpoint("test-b", 0),
            Err(Fault::BadBreakpoint {
                label: "test-b".to_owned(),
                n: 0
            })
        );
        assert_eq!(
            machine.set_breakpoint("no-such-label", 1),
            Err(Fault::UnknownLabel {
                label: "no-such-label".to_owned()
            })
        );
        assert_eq!(
            machine.set_breakpoint("gcd-done", 1),
            Err(Fault::BadBreakpoint {
                label: "gcd-done".to_owned(),
                n: 1
            })
        );
        machine.set_breakpoint("test-b", 4).unwrap();
        machine.set_breakpoint("test-b", 4).unwrap();
        assert!(matches!(machine.start().unwrap(), Stop::Breakpoint { .. }));
        machine.cancel_all_breakpoints();
        assert_eq!(machine.proceed().unwrap(), Stop::End);
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(4));
    }
}
