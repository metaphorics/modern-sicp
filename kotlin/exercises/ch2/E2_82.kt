// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.82

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.82: generalize `apply-generic` to coerce in the multi-
 * argument case by trying, in turn, to coerce every argument to the type
 * of the first argument, then to the type of the second, and so on.
 * [applyGenericMultiCoerce] implements that strategy. The exercise then
 * asks where it is not general enough: [ex_2_82] shows a table holding a
 * suitable mixed-type operation that the strategy never tries, because it
 * coerces all arguments uniformly or not at all -- never a subset.
 */
public fun ex_2_82(): Pair<Boolean, Boolean> = throw PendingSolution()
