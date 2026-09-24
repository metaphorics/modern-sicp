// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.56

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.56 (Class A): extend the differentiator with the power rule
 * `d(u^n)/dx = n u^(n-1) du/dx`, adding a fifth case to a local copy of
 * the section's `Expr`/`deriv` named `PowExpr`/`derivPow` (kept local to
 * this exercise, per the section's one-representation-per-exercise
 * convention: see `AlgExpr.kt` for the shared, unmodified base). Build in
 * the rules that anything to the power 0 is 1 and anything to the power 1
 * is the base itself. Exponentiation prints as `(** base n)`.
 *
 * The scaffold returns the printed derivative of `x^3` with respect to
 * `x`.
 */
public fun ex_2_56(): String = throw PendingSolution()
