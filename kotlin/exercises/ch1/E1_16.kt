// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.16

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.16, adapted for a host with no blanket tail-call guarantee:
 * design a procedure that evolves an iterative exponentiation process
 * that uses successive squaring and a logarithmic number of steps, as
 * `fastExpt` does. Hint: using `(b^(n/2))^2 = (b^2)^(n/2)`, keep an
 * additional state variable `a` alongside `n` and `b` such that the
 * product `a . b^n` never changes from state to state; `a` starts at 1,
 * and the answer is the value of `a` at the end. The statement lives in
 * the section 1.2 chapter text.
 *
 * The scaffold returns `b^n` computed by the iterative process.
 */
public fun ex_1_16(
    b: Long,
    n: Long,
): Long = throw PendingSolution()
