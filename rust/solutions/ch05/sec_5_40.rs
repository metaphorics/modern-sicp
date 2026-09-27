// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.40: the compile-time
//! environment is threaded through every code generator.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Config, Linkage, compile_program, new_state};

mod ex_5_40 {
    //! Exercise 5.40: a lambda extends the compile-time environment
    //! with its parameter frame; variable references see that frame
    //! and the frames outside it.

    use super::*;

    const EXAMPLE: &str =
        "((lambda (x y) (lambda (a b c d e) ((lambda (y z) (* x y z)) (* a b x) (+ c d x)))) 3 4)";

    pub fn ex_5_40() -> Result<Vec<String>, Fault> {
        let trace = Rc::new(RefCell::new(Vec::<String>::new()));
        let sink = Rc::clone(&trace);
        let cfg = Config {
            trace: Some(Rc::new(move |frames, name| {
                let rendered = frames
                    .iter()
                    .map(|frame| format!("({})", frame.join(" ")))
                    .collect::<Vec<_>>()
                    .join(" ");
                sink.borrow_mut().push(format!("{name} in ({rendered})"));
            })),
            ..ch05::sec_5_5::default_config()
        };
        let _ = compile_program(&cfg, &new_state(), EXAMPLE, &Linkage::Next)?;
        let trace = trace.borrow().clone();
        assert!(
            trace
                .iter()
                .any(|line| line == "x in ((y z) (a b c d e) (x y))")
        );
        assert!(
            trace
                .iter()
                .any(|line| line == "z in ((y z) (a b c d e) (x y))")
        );
        Ok(trace)
    }

    #[test]
    fn ex_5_40_check() -> Result<(), Fault> {
        let lines = ex_5_40()?;
        assert!(lines.iter().any(|line| line.starts_with("x in")));
        Ok(())
    }
}
