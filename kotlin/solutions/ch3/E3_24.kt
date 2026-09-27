// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.24

package sicp.ch3.exercises

import sicp.runtime.Value

/**
 * The table constructor with the caller's key test: ``equality'' of
 * keys is whatever sameKey says, and the returned table's lookup and
 * insert run their assoc scan through that predicate.
 */
public fun makeTable(sameKey: (Value, Value) -> Boolean): Table = Table(sameKey)
