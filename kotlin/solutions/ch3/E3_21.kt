// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

/**
 * The original interpreter printed a queue as one pair of its two
 * pointers: the front chain in the car, the rear cell's single item in
 * the cdr position -- ((a b c d) d) for a queue of a, b, c, d, and
 * ((a) a) for a queue of a alone. `printQueue` walks the front chain;
 * `benView` rebuilds the raw pair view out of the two cells the class
 * exposes, with () standing in for an empty pointer.
 */
public fun benView(queue: Queue): String {
    val front = queue.frontCell()?.toString() ?: "()"
    val rear = queue.rearCell()?.car?.toString() ?: "()"
    return "($front $rear)"
}
