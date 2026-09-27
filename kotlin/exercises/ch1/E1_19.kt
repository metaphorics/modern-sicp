// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19

package sicp.ch1.exercises

import sicp.runtime.PendingSolution
import java.math.BigInteger

/**
 * Exercise 1.19, adapted for a fixed-width host: there is a clever
 * algorithm for computing the Fibonacci numbers in a logarithmic number
 * of steps. The transformation `T(a, b) = (a + b, a)` is the special case
 * `p = 0, q = 1` of a family `T_pq(a, b) = (bq + aq + ap, bp + aq)`.
 * Applying `T_pq` twice has the same effect as one `T_p'q'`; find `p'`
 * and `q'` in terms of `p` and `q`, and use the result to compute `T^n`
 * by successive squaring, completing a `fibLog` procedure that runs in a
 * logarithmic number of steps. `Fib(n)` grows past `Long.MAX_VALUE`
 * before `n = 100`, so this edition states `fibLog` over `BigInteger`.
 * The statement lives in the section 1.2 chapter text.
 *
 * The scaffold returns `Fib(n)`.
 */
public fun ex_1_19(n: Long): BigInteger = throw PendingSolution()
