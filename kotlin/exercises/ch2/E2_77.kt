// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.77

package sicp.ch2.exercises

/**
 * Exercise 2.77: Louis Reasoner tries to evaluate `magnitude(z)` where `z`
 * is the two-level number of Figure 2.24 -- here `Complex(Rect(3.0, 4.0))`
 * -- and gets a `GenError.NoMethod` for `magnitude` on `(complex)`, because
 * the complex package installs `add`, `sub`, `mul`, and `div` but none of
 * the selectors. Alyssa's fix: install the four selectors at the complex
 * level, each forwarding to the representation level the outer tag was
 * hiding. Evaluating `magnitude` then dispatches twice -- once at the
 * complex level, once at the rectangular level -- which [ex_2_77] counts.
 */
public fun ex_2_77(): Int = throw PendingExercise()
