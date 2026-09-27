// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 3.20: the procedural pair of 3.3.1 as an interface over
 * captured slots. SICP's exercise traces environment diagrams for the
 * Scheme dispatch version; this edition traces the same shared state in
 * Kotlin terms: after `alias = x` and `alias.setCar(VInt(17))`, `x.car()`
 * answers 17 because both names hold the same object over the same
 * captured `var` slots, while a fresh `proceduralCons` pair has slots of
 * its own.
 */
public interface PairProc {
    public fun car(): Value

    public fun cdr(): Value

    public fun setCar(v: Value)

    public fun setCdr(v: Value)
}

/** The book's mutable cons: two captured slots behind one object. */
public fun proceduralCons(
    x: Value,
    y: Value,
): PairProc = throw PendingSolution()
