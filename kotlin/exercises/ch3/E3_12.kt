// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.VPair
import sicp.runtime.Value

/**
 * Exercise 3.12: `append` is the constructor from 2.2.1 -- it builds a
 * fresh list by consing the elements of `x` onto `y`, touching no pair of
 * `x`. `appendBang` is the mutator version: it splices `y` in after the
 * last pair of `x` and returns `x` itself. The question is which pairs
 * `z = append(x, y)` shares with `x` and which pairs `w =
 * appendBang(x, y)` shares, read off what `x.cdr` prints after each call.
 */
public fun append(
    x: Value,
    y: Value,
): Value = throw PendingSolution()

/** The last pair of the nonempty chain `x`. */
public fun lastPair(x: VPair): VPair = throw PendingSolution()

/** The book's append!: splice `y` onto the end of `x` by mutation. */
public fun appendBang(
    x: VPair,
    y: VPair,
): VPair = throw PendingSolution()
