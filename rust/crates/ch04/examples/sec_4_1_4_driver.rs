// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.4: running the evaluator as a program. The global scope
//! holds the built-in operations under their source names, and a
//! checked program runs to its ordered transcript: a definition
//! prints nothing on its own, calls print their values in evaluation
//! order, and a trap stops the run after the output already written.

use ch04::sec_4_1::{run, run_program, run_source};
use sicp_runtime::host::admit;

const PRIMITIVES: &str = "\
fn main() {
    println!(\"{}\", 1 + 6);
    println!(\"{}\", 1 + 2 * 3);
}
";

const SESSION: &str = "\
fn append(xs: Vec<i64>, ys: Vec<i64>) -> Vec<i64> {
    let mut out = xs;
    for y in ys {
        out.push(y);
    }
    out
}

fn main() {
    let joined = append(vec![1, 2, 3], vec![4, 5, 6]);
    println!(\"{:?}\", joined);
    println!(\"{:?}\", (10, (20, 30)));
}
";

const WHOLE: &str = "\
fn square(x: i64) -> i64 {
    x * x
}

fn main() {
    println!(\"{}\", square(6));
    println!(\"{}\", \"done\");
}
";

const STOPPED: &str = "\
fn main() {
    println!(\"{}\", 36);
    let d = 0;
    println!(\"{}\", 1 / d);
    println!(\"{}\", 99);
}
";

fn main() {
    // The global scope answers the built-in operations by name.
    let output = run_program(PRIMITIVES);
    println!("{output}");
    // => 7
    // => 7
    assert_eq!(output, "7\n7\n");

    // The driver session's shapes: the append definition joins two
    // sequences, and a nested pair prints as the structure it is.
    let output = run_program(SESSION);
    println!("{output}");
    // => [1, 2, 3, 4, 5, 6]
    // => (10, (20, 30))
    assert_eq!(output, "[1, 2, 3, 4, 5, 6]\n(10, (20, 30))\n");

    // A whole program runs the same way with the printer's output: a
    // definition prints nothing, the calls print their values.
    let output = run_program(WHOLE);
    println!("{output}");
    // => 36
    // => done
    assert_eq!(output, "36\ndone\n");

    // The transcript is the same transcript every engine of the
    // edition observes, byte for byte.
    assert_eq!(
        run_source(SESSION).expect("admitted").stdout,
        run_program(SESSION)
    );

    // A trap prints one line of context on the error channel and
    // stops; everything before it stands in the transcript.
    let outcome = run(&admit(STOPPED).expect("checked"));
    println!("{}", outcome.stdout);
    // => 36
    assert_eq!(outcome.stdout, "36\n");
    assert!(outcome.trap.is_some());
}
