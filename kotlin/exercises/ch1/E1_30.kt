// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.30

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.30: the `sum` procedure of section 1.3.1 generates a linear
 * recursive process. Rewrite it so the sum is performed iteratively, by
 * filling in the missing pieces of
 *
 * ```
 * fun sumIterative(term: (Long) -> Long, a: Long, next: (Long) -> Long, b: Long): Long {
 *     tailrec fun iter(a: Long, result: Long): Long =
 *         if (⟨??⟩) ⟨??⟩ else iter(⟨??⟩, ⟨??⟩)
 *     return iter(⟨??⟩, ⟨??⟩)
 * }
 * ```
 *
 * The statement lives in the section 1.3 chapter text.
 *
 * The scaffold returns `sumIterative(cube, 1, inc, 10)`, the same
 * `sum-cubes` total section 1.3.1 computed recursively.
 */
public fun ex_1_30(): Long = throw PendingSolution()
