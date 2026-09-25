// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4.4.1: the driver loop and instantiation: filing an
//! `assert!`, answering queries, and printing unbound variables as
//! `?name`.

use ch04::sec_4_4::microshaft;

fn main() {
    let engine = microshaft();

    // A query streams its answers; an assertion is filed silently.
    let session = engine.session(&[
        "(supervisor ?name (Bitdiddle Ben))",
        "(assert! (job (Bitdiddle Ben) (computer wizard)))",
        "(salary (Bitdiddle Ben) ?amount)",
    ]);
    println!("@{session}");
    assert!(session.contains(
        ";;; Query input: (supervisor ?name (Bitdiddle Ben))\n\
         ;;; Query results:\n\
         (supervisor (Hacker Alyssa P) (Bitdiddle Ben))\n"
    ));
    assert!(session.contains("Assertion added to data base.\n"));
    assert!(session.contains("(salary (Bitdiddle Ben) 60000)\n"));

    // A rule is filed the same way and answers queries afterwards.
    let session = engine.session(&[
        "(assert! (rule (same ?x ?x)))",
        "(same (Bitdiddle Ben) (Bitdiddle Ben))",
    ]);
    assert!(session.contains("Assertion added to data base.\n"));
    assert!(session.contains("(same (Bitdiddle Ben) (Bitdiddle Ben))\n"));
}
