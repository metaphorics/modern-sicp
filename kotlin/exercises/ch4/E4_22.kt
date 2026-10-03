// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.22

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.22: extend the analyzer of 4.1.7 to support `let`. The
 * analyzer's extension seam is `analyze`: a `LetAnalyzer` checks for the
 * typed `LetE` node the parser produced before falling back to
 * `baseAnalyze`, and because every analysis of a subexpression recurses
 * through `analyze`, the clause fires at every nesting depth. Handle
 * `let` directly in the analysis phase -- each initializer analyzed once,
 * the body analyzed once, execution only extending the frame and running
 * -- rather than re-deriving to the 4.6 `GLam`/`GApp` application.
 *
 * [letTranscript] runs the shadowing probe on the analyzer: the plain
 * the `GLet(x = 7)` body answers 7; an inner `let` shadowing the outer
 * binding => 2; an initializer reading the outer binding => 5; and a
 * `let` inside an analyzed procedure body, `(f 10)` with body
 * the nested `GLet(y = x * x)` body answers 101.
 */
public fun letTranscript(): String = throw PendingSolution()
