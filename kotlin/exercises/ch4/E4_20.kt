// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.20: `letrec` as a derived expression. Part (a): the parser
 * leaves every `letrec` an application of the head symbol, so the
 * evaluator's `step` recognizes `(letrec ((name init) ...) body ...)` and
 * lowers it like 4.16's scan-out: one `let` reserving every name with
 * `*unassigned*`, one `set!` per name in source order, then the body, so
 * mutual recursion works and only genuinely premature reads fail the typed
 * check. Part (b): Louis Reasoner wants `letrec` to have the same
 * properties as `let` -- but a plain `let` evaluates its initializers
 * before any frame holds the names, so the recursive `fact` reference
 * fails unbound; reserving the names first is what makes recursion legal.
 *
 * Expected answers: the even?/odd? `letrec` answers `#t` on 10; the
 * factorial `letrec` answers 3628800 on 10; an initializer reading a later
 * binding fails with `Error: type mismatch: y is read before it is
 * assigned`; the same recursion under a plain `let` fails with
 * `Error: unbound variable: fact`.
 */
public fun letrecEvenOddTranscript(): String = throw PendingSolution()

public fun letrecFactTranscript(): String = throw PendingSolution()

public fun letrecPrematureReadTranscript(): String = throw PendingSolution()

public fun plainLetUnboundTranscript(): String = throw PendingSolution()
