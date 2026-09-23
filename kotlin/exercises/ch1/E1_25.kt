// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.25

package sicp.ch1.exercises

import sicp.runtime.PendingSolution
import java.math.BigInteger

/**
 * Exercise 1.25: Alyssa P. Hacker complains that `expmod` does extra work
 * that a simpler `remainder(fastExpt(base, exp), m)` would avoid. Is she
 * correct? Would this procedure serve as well for the fast prime tester?
 * The statement lives in the section 1.2 chapter text.
 *
 * The scaffold computes `expmod(7, 200, 13)` three ways: the section's
 * `expmod`, Alyssa's simplification over checked-nothing `Long`
 * arithmetic (which the host silently wraps on overflow), and Alyssa's
 * simplification over `BigInteger` (which never overflows). It returns
 * the triple `(correct, naiveLongWrapped, naiveBigInteger)`.
 */
public fun ex_1_25(): Triple<Long, Long, Long> = throw PendingSolution()
