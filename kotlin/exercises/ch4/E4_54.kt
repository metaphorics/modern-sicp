// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.54

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.54: `require` as a special form. The variant evaluator
 * recognizes `require` at the application head and fails outright when
 * the predicate is false, instead of relying on the user-defined
 * procedure that calls `(amb)`.
 *
 * Expected answers: the evenness-filtered choice delivers 2 then 4 and
 * is then exhausted -- the false predicates prune 1, 3, and 5 -- and a
 * satisfied requirement answers `ok`.
 */
public fun requireFilteredEvens(): List<String> = throw PendingSolution()

public fun requireSatisfied(): String = throw PendingSolution()
