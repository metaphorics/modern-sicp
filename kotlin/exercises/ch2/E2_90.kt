// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.90

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.90: a polynomial system efficient for both sparse and dense
 * polynomials. The two term-list representations sit behind one interface
 * -- [TermListRepresentation] -- and the term-list algorithms
 * [addTermsVia]/[mulTermsVia] are written once against it. The sparse
 * provider stores `(order coeff)` terms; the dense provider stores plain
 * coefficients and derives each order from the position, exactly the two
 * representations the section contrasts.
 */
public fun ex_2_90(): Boolean = throw PendingSolution()
