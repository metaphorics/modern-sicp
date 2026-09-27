// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.26

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.26: Louis Reasoner rewrites `expmod` to use an explicit
 * multiplication instead of calling `square`, and his `fastPrime` test
 * runs slower, not faster. Eva Lu Ator says this turns the
 * Theta(log n) process into a Theta(n) process. Explain, by counting how
 * many times each version of `expmod` is called while computing
 * `expmod(4, exponent, 97)`. The statement lives in the section 1.2
 * chapter text.
 *
 * The scaffold returns the pair (calls with `square`, calls with an
 * explicit doubled multiplication) for `exponent = 64`.
 */
public fun ex_1_26(): Pair<Int, Int> = throw PendingSolution()
