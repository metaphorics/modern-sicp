// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16

package sicp.ch4.exercises

import sicp.runtime.Expr
import sicp.runtime.PendingSolution

/**
 * Exercise 4.16: scan out internal definitions. Alyssa P. Hacker proposes
 * `scan-out-defines`: a procedure body containing `define` forms is
 * rewritten into one `let` that reserves every defined name with the
 * section's `*unassigned*` marker, followed by `set!` assignments in
 * source order and the rest of the body; a body without internal defines
 * is unchanged. Part (b): installing the scan in `make-procedure` scans
 * once per procedure creation, while installing it in `apply` would
 * re-scan the body on every call -- `make-procedure` is the better place.
 * Part (c): the evaluator raises the typed premature-read fault when a
 * name still holding the marker is read: this edition picks
 * `TypeMismatch`, since the binding exists and what failed is reading a
 * value no proper operation accepts.
 *
 * Expected answers: the statement's mutual even?/odd? recursion answers
 * `#t` on 10 under the scan-out; a body whose eager initializer reads a
 * later define fails with `Error: type mismatch: a is read before it is
 * assigned`, while the base evaluator fails the same program with
 * `Error: unbound variable: a`.
 */
public fun mutualRecursionTranscript(): String = throw PendingSolution()

public fun prematureReadTranscript(): String = throw PendingSolution()

public fun basePrematureTranscript(): String = throw PendingSolution()

public fun scanOutDefines(body: List<Expr>): List<Expr> = throw PendingSolution()
