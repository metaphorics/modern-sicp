// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.20

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.20 is about Scheme itself, so this edition replaces it; the
 * exercise below keeps the number and teaches the same section material.
 * SICP's original asks how many times `remainder` runs when `gcd` is
 * traced under normal-order versus applicative-order evaluation, a
 * distinction that only exists for a host with a lazy substitution
 * semantics; Kotlin, like every host language in this book, only
 * evaluates eagerly. Instrument the eager `gcd` of section 1.2.5 to count
 * every call it makes to `remainder` while evaluating `gcd(206, 40)`. The
 * statement lives in the section 1.2 chapter text.
 *
 * The scaffold returns that count.
 */
public fun ex_1_20(): Int = throw PendingSolution()
