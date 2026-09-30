// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.24

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PendingSolution

/**
 * Exercise 3.24 lets a table caller choose key equality instead of using
 * structural datum equality. For example, a predicate can place nearby
 * whole numbers in the same table slot.
 */
public fun makeTable(sameKey: (Datum, Datum) -> Boolean): Table = throw PendingSolution()
