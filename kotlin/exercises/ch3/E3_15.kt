// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.VPair

/**
 * Exercise 3.15: `setToWow` replaces the car of `x`'s first pair with the
 * symbol `wow`. The question is why applying it to `z1 = cons(x, x)`
 * shows the change through both the car and the cdr, while applying it to
 * `z2 = cons((a b), (a b))` shows it through the car only -- draw the two
 * box-and-pointer diagrams and state what each name observes.
 */
public fun setToWow(x: VPair): VPair = throw PendingSolution()
