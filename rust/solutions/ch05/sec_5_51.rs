// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.51: the explicit-control
//! evaluator of 5.4 translated into C and built by the system C
//! compiler.
//!
//! The translation is the controller itself: the registers are
//! fields of one `eceval` function's statics, every `ev-` entry point
//! of 5.4.1 to 5.4.4 is a C label, `(assign continue (label l))`
//! stores a label address, `(goto (reg continue))` is `goto *R_continue`
//! (the GNU computed-goto extension the system compiler accepts), and
//! the stack is an array with the book's push/pop discipline and
//! monitored counters. The object world of pairs, symbols,
//! environment frames, and the primitive table is the run-time
//! support the exercise requires.

use std::process::Command;

use ch05::sec_5_2::Fault;

mod ex_5_51 {
    //! Exercise 5.51: the C translation answers the book's factorial
    //! session: `ok`, then `120`.

    use super::*;

    const ECEVAL_C: &str = include_str!("eceval_5_51.c");

    const PROGRAM: &str = "(define (factorial n)\n  (if (= n 1)\n      1\n      (* (factorial (- n 1)) n)))\n(factorial 5)\n";

    fn compile_and_run() -> Result<String, Fault> {
        let dir = std::env::temp_dir().join(format!("sicp_rust_5_51_{}", std::process::id()));
        std::fs::create_dir_all(&dir)
            .map_err(|error| Fault::Parse(format!("scratch dir: {error}")))?;
        let source = dir.join("eceval.c");
        let binary = dir.join("eceval");
        let program = dir.join("program.scm");
        std::fs::write(&source, ECEVAL_C)
            .map_err(|error| Fault::Parse(format!("write c: {error}")))?;
        std::fs::write(&program, PROGRAM)
            .map_err(|error| Fault::Parse(format!("write program: {error}")))?;
        let build = Command::new("cc")
            .arg("-O1")
            .arg("-o")
            .arg(&binary)
            .arg(&source)
            .output()
            .map_err(|error| Fault::Parse(format!("cc spawn: {error}")))?;
        if !build.status.success() {
            let text = String::from_utf8_lossy(&build.stderr).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            return Err(Fault::Parse(format!(
                "the C translation failed to build: {text}"
            )));
        }
        let run = Command::new(&binary)
            .arg(&program)
            .output()
            .map_err(|error| Fault::Parse(format!("run spawn: {error}")))?;
        if !run.status.success() {
            let text = String::from_utf8_lossy(&run.stderr).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            return Err(Fault::Parse(format!(
                "the C translation failed with status {}: {text}",
                run.status
            )));
        }
        let text = String::from_utf8_lossy(&run.stdout).to_string();
        let _ = std::fs::remove_dir_all(&dir);
        Ok(text)
    }

    pub fn ex_5_51() -> Result<Vec<String>, Fault> {
        let output = compile_and_run()?;
        assert!(output.contains("ok"), "{output}");
        assert!(output.contains("120"), "{output}");
        Ok(vec![output])
    }

    #[test]
    fn ex_5_51_check() -> Result<(), Fault> {
        let lines = ex_5_51()?;
        assert!(lines[0].contains("ok"));
        assert!(lines[0].contains("120"));
        Ok(())
    }
}
