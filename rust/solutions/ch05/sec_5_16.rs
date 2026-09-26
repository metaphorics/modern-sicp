// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.16: instruction tracing,
//! switchable while the machine is stopped.

use ch05::sec_5_2::gcd_machine;
use sicp_runtime::Value;

mod ex_5_16 {
    //! Exercise 5.16: before each executed instruction, print its
    //! text; `trace-on` and `trace-off` switch the printing.

    use super::*;

    /// Tracing the GCD machine on 12 and 8 prints one line per
    /// executed instruction: fourteen, the three passes of the loop
    /// plus the final test and branch, and the answer is untouched.
    #[test]
    fn ex_5_16_traced_gcd() {
        let mut machine = gcd_machine();
        machine.set_register("a", Value::Int(12)).unwrap();
        machine.set_register("b", Value::Int(8)).unwrap();
        machine.set_trace(true);
        machine.start().unwrap();
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(4));
        assert_eq!(
            machine.transcript(),
            [
                "(test (op =) (reg b) (const 0))".to_owned(),
                "(branch (label gcd-done))".to_owned(),
                "(assign t (op rem) (reg a) (reg b))".to_owned(),
                "(assign a (reg b))".to_owned(),
                "(assign b (reg t))".to_owned(),
                "(goto (label test-b))".to_owned(),
                "(test (op =) (reg b) (const 0))".to_owned(),
                "(branch (label gcd-done))".to_owned(),
                "(assign t (op rem) (reg a) (reg b))".to_owned(),
                "(assign a (reg b))".to_owned(),
                "(assign b (reg t))".to_owned(),
                "(goto (label test-b))".to_owned(),
                "(test (op =) (reg b) (const 0))".to_owned(),
                "(branch (label gcd-done))".to_owned(),
            ]
        );
    }

    /// With tracing off the same run prints nothing; and a machine
    /// whose tracing is switched off between stops stays quiet from
    /// there on.
    #[test]
    fn ex_5_16_trace_off_prints_nothing() {
        let mut machine = gcd_machine();
        machine.set_register("a", Value::Int(12)).unwrap();
        machine.set_register("b", Value::Int(8)).unwrap();
        machine.start().unwrap();
        assert!(machine.transcript().is_empty());

        machine.set_register("a", Value::Int(12)).unwrap();
        machine.set_register("b", Value::Int(8)).unwrap();
        machine.set_trace(true);
        machine.set_trace(false);
        machine.start().unwrap();
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(4));
        assert!(machine.transcript().is_empty());
    }
}
