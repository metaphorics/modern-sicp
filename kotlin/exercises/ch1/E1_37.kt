// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.37

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.37: (a) an infinite continued fraction is an expression of
 * the form `N1/(D1 + N2/(D2 + N3/(D3 + ...)))`. As an example, the
 * infinite continued fraction with the `N_i` and the `D_i` all equal to
 * 1 produces `1/phi`, where `phi` is the golden ratio. One way to
 * approximate an infinite continued fraction is to truncate the
 * expansion after a given number of terms: a `k`-term finite continued
 * fraction. Suppose `n` and `d` are procedures of one argument (the term
 * index `i`) that return the `N_i` and `D_i` of the continued fraction.
 * Define a procedure `contFrac` such that evaluating `contFrac(n, d, k)`
 * computes the value of the `k`-term finite continued fraction. Check
 * your procedure by approximating `1/phi` using
 * `contFrac({i -> 1.0}, {i -> 1.0}, k)` for successive values of `k`.
 * How large must `k` be to get an approximation accurate to 4 decimal
 * places? (b) If your `contFrac` procedure generates a recursive
 * process, write one that generates an iterative process. If it
 * generates an iterative process, write one that generates a recursive
 * process. The statement lives in the section 1.3 chapter text.
 *
 * The scaffold returns the smallest `k` accurate to 4 decimal places,
 * and the recursive and iterative approximations at that `k`.
 */
public fun ex_1_37(): Triple<Long, Double, Double> = throw PendingSolution()
