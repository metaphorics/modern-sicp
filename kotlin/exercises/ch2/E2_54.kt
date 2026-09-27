// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.54

package sicp.ch2.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 2.54 (Class A): implement `equal?` recursively over this
 * edition's sealed `Value` hierarchy, adapted from the book's statement
 * in terms of `eq?`, `car`, and `cdr`: two values are `myEqual` if they
 * are both symbols and `==`, both the empty list, or both pairs whose
 * cars are recursively `myEqual` and whose cdrs are recursively
 * `myEqual`.
 */
public fun myEqual(
    a: Value,
    b: Value,
): Boolean = throw PendingSolution()

/** `myEqual` applied to the book's two worked examples, in order. */
public fun ex_2_54(): List<Boolean> = throw PendingSolution()
