// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.32

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.32: (a) show that `sum` and `product` (exercise 1.31) are
 * both special cases of a still more general notion called `accumulate`
 * that combines a collection of terms, using a general accumulation
 * function: `accumulate(combiner, nullValue, term, a, next, b)`.
 * `accumulate` takes the same term and range specifications as `sum` and
 * `product`, together with a `combiner` of two arguments that specifies
 * how the current term combines with the accumulation of the preceding
 * terms, and a `nullValue` that specifies the base value when the terms
 * run out. Write `accumulate` and show how `sum` and `product` can both
 * be defined as simple calls to `accumulate`. (b) If your `accumulate`
 * generates a recursive process, write one that generates an iterative
 * process, or vice versa. The statement lives in the section 1.3 chapter
 * text.
 *
 * The scaffold returns `sum-cubes(1, 10)` and `factorial(6)`, both
 * defined as calls to `accumulate`.
 */
public fun ex_1_32(): Pair<Long, Long> = throw PendingSolution()
