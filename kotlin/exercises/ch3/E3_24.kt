// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.24

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 3.24: the section's makeTable tests keys with equalv unless
 * the constructor was handed a different predicate. Design the table
 * constructor that takes the caller's sameKey predicate, so that
 * "equality" of keys is whatever the table's user says it is -- for
 * instance, numbers within some tolerance. The returned table's
 * lookup and insert use that test.
 */
public fun makeTable(sameKey: (Value, Value) -> Boolean): Table = throw PendingSolution()
