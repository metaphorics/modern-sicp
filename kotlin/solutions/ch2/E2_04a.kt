// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4a

package sicp.ch2.exercises

// consFn, carFn, and cdrFn are exercise 2.4's public declarations, reused here from the same package.

public fun ex_2_04a(): Boolean = carFn(consFn(3L, 4L)) == 3L && cdrFn(consFn(3L, 4L)) == 4L
