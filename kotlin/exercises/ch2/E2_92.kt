// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.92

package sicp.ch2.exercises

/**
 * Exercise 2.92: impose an ordering on variables so polynomial addition
 * and multiplication work for polynomials in different variables. The
 * dominant variable is canonical: a polynomial in a lower variable lifts
 * into the dominant one as a single order-zero term, so no expansion
 * happens. Coefficient arithmetic learns one new move alongside -- an
 * integer constant meets a polynomial by becoming a constant polynomial
 * in the same variable, the coercion the section's footnote asks for.
 */
public fun ex_2_92(): Pair<Boolean, Boolean> = throw PendingExercise()
