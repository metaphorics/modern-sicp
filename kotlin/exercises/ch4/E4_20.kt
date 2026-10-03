// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.20: `GLetRec` as a derived expression. Part (a): the
 * rewrite recognizes the recursive-binding node and lowers it like
 * 4.16's scan-out: one binding form reserving every name with the
 * unassigned marker, one assignment per name in source order, then the
 * body, so mutual recursion works and only genuinely premature reads
 * raise the typed fault. Part (b): Louis Reasoner wants the recursive
 * binding to have the same properties as `GLet` -- but a plain `GLet`
 * evaluates its initializers before any frame holds the names, so the
 * recursive `fact` reference answers null; reserving the names first is
 * what makes recursion legal.
 *
 * Expected answers: the even?/odd? recursion answers `true` on 10; the
 * factorial recursion answers 3628800 on 10; an initializer reading a
 * later binding raises `UnassignedRead`; the same recursion under a plain
 * `GLet` answers null.
 */
public fun letrecEvenOddTranscript(): String = throw PendingSolution()

/** The factorial recursion. => "3628800\n" */
public fun letrecFactTranscript(): String = throw PendingSolution()

/** An initializer reading a later binding raises the typed fault.
 * => "UnassignedRead" */
public fun letrecPrematureReadTranscript(): String = throw PendingSolution()

/** The same recursion under a plain `GLet` answers the kernel's fault.
 * => "null" */
public fun plainLetUnboundTranscript(): String = throw PendingSolution()
