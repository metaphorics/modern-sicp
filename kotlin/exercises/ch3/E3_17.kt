// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 3.17: a corrected `countDistinctPairs` returns the number of
 * distinct pairs in any structure, never counting a shared pair twice.
 * The host differs from Scheme here: the standard library has no
 * identity-keyed set, so the auxiliary "already counted" structure must
 * be one you scan with `===` yourself.
 */
public fun countDistinctPairs(x: Value): Int = throw PendingSolution()
