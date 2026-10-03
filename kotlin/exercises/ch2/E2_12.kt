// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.12

package sicp.ch2.exercises

/**
 * Exercise 2.12: define a constructor `makeCenterPercent` that takes a
 * center and a percentage tolerance, expressed as a fraction such as `0.1`
 * for ten percent, and produces the desired interval. You must also define
 * a selector `percent` that produces the percentage tolerance for a given
 * interval. `Interval`, `makeInterval`, `lowerBound`, and `upperBound` are
 * exercise 2.7's public declarations; `width` is exercise 2.9's, and
 * `center` is defined below, the same as the main text's; all are reused
 * here from the same package. The statement lives in the section 2.1
 * chapter text.
 *
 * The scaffold returns `makeCenterPercent(50.0, 0.1)`, a center of 50 with
 * ten percent tolerance.
 */
public fun center(i: Interval): Double = (i.lowerBound + i.upperBound) / 2.0

public fun ex_2_12(): Interval = throw PendingExercise()
