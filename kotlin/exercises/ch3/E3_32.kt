// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.32

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.32: the actions of one time segment sit in a queue, so a
 * segment's actions run in the order they were added to the agenda,
 * first in, first out. Explain why this order must be used: in
 * particular, trace an and-gate whose inputs change from 0, 1 to 1, 0
 * in the same segment, and say how the behavior would differ if a
 * segment's actions came off a plain list, added and removed only at
 * the front, last in, first out.
 *
 * This edition pins the order observably: [sameSegmentRunOrder] adds
 * two actions to one agenda segment and answers the order `propagate`
 * runs them in.
 */
public fun sameSegmentRunOrder(): List<String> = throw PendingSolution()
