// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4

package sicp.ch2.exercises

/** Given by the exercise statement. */
public fun <T> consFn(
    x: T,
    y: T,
): ((T, T) -> T) -> T = { m -> m(x, y) }

/** Given by the exercise statement. */
public fun <T> carFn(z: ((T, T) -> T) -> T): T = z { p, _ -> p }

/**
 * `consFn(x, y)` is the function `{ m -> m(x, y) }`; applying it to a
 * selector that returns its second argument recovers `y`, by the same
 * substitution [carFn] relies on to recover the first.
 */
public fun <T> cdrFn(z: ((T, T) -> T) -> T): T = z { _, q -> q }

public fun ex_2_04(): Long = cdrFn(consFn(3L, 4L))
