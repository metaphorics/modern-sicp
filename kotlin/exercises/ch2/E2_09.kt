// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.9

package sicp.ch2.exercises

/**
 * Exercise 2.9: the width of an interval is half of the difference between
 * its upper and lower bounds. The width is a measure of the uncertainty of
 * the number specified by the interval. For some arithmetic operations the
 * width of the result of combining two intervals is a function only of the
 * widths of the argument intervals, whereas for others the width of the
 * combination is not a function only of the widths of the argument
 * intervals being combined. Show that the width of the sum (or difference)
 * of two intervals is a function only of the widths of the intervals being
 * added (or subtracted). Give examples to show that this is not true for
 * multiplication or division. `Interval`, `makeInterval`, `lowerBound`,
 * `upperBound`, `addInterval`, and `mulInterval` are exercise 2.7's public
 * declarations, reused here from the same package. The statement lives in
 * the section 2.1 chapter text.
 *
 * A data-class `Interval` places no invariant on which bound is smaller;
 * `width` can therefore go negative for a reversed interval instead of
 * signaling an error. This exercise's solution discusses that choice in
 * its rationale rather than in `Interval` itself.
 *
 * The scaffold returns the width of the product of two width-1 intervals
 * at two different centers, showing the two product widths differ despite
 * the equal input widths.
 */
public fun ex_2_09(): Pair<Double, Double> = throw PendingExercise()
