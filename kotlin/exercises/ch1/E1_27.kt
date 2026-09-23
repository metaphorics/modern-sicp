// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.27

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.27: demonstrate that the Carmichael numbers 561, 1105, 1729,
 * 2465, 2821, and 6601 really do fool the Fermat test. Write a procedure
 * that tests whether `a^n` is congruent to `a` modulo `n` for every
 * `a < n`, and try it on the given numbers. The statement lives in the
 * section 1.2 chapter text.
 *
 * The scaffold returns a map from each Carmichael number to whether it
 * fools the test for every `a` from 1 up to (but not including) itself.
 */
public fun ex_1_27(): Map<Long, Boolean> = throw PendingSolution()
