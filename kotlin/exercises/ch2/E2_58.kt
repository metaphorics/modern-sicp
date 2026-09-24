// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.58

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.58 (Class A): differentiate expressions written in ordinary
 * infix notation instead of the section's prefix `Sum`/`Product`. Part a)
 * assumes every expression is fully parenthesized, such as
 * `"(x + (3 * (x + (y + 2))))"`. Part b) drops that assumption and
 * requires the standard precedence rule that `*` binds tighter than `+`,
 * such as `"x + 3 * (x + y + 2)"`. Both parts parse into this section's
 * shared `Expr` (see `AlgExpr.kt`), so `deriv` differentiates the result
 * unchanged.
 *
 * The scaffold returns the printed derivative of `"x + 3 * (x + y + 2)"`
 * with respect to `x`, parsed with the precedence-aware parser of part b.
 */
public fun ex_2_58(): String = throw PendingSolution()
