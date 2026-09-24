// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.1

package sicp.ch3.exercises

/** A closure over one mutable cell: each call adds `amount` to `sum` and returns the new total. */
public fun makeAccumulator(start: Long): (Long) -> Long {
    var sum = start
    return { amount ->
        sum += amount
        sum
    }
}
