// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.57 (Class A): extend the differentiator to handle sums and
 * products of two or more terms, without changing `deriv`'s own shape at
 * all: only `addend`/`augend` and `multiplier`/`multiplicand` change, so
 * that the augend of a sum is the sum of the rest of the terms (a single
 * remaining term stands for itself) and likewise for a product's
 * multiplicand. Kept as its own local `NaryExpr`/`derivN`, per the
 * section's one-representation-per-exercise convention.
 *
 * The scaffold returns the printed derivative of `(* x y (+ x 3))` with
 * respect to `x`.
 */
public fun ex_2_57(): String = throw PendingSolution()
