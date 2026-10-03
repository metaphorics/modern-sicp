// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.14

package sicp.ch2.exercises

/**
 * Exercise 2.14: demonstrate that Lem is right: `par1` and `par2` (below,
 * the main text's own two parallel-resistance formulas) give different
 * answers. Investigate the behavior of the system on a variety of
 * arithmetic expressions. Make some intervals `a` and `b`, and use them in
 * computing the expressions `a / a` and `a / b`. You will get the most
 * insight by using intervals whose width is a small percentage of the
 * center value. Examine the results of the computation in center-percent
 * form (exercise 2.12). `addInterval`, `mulInterval`, `divInterval`, and
 * `makeInterval` are exercise 2.7's public declarations; `makeCenterPercent`
 * and `percent` are exercise 2.12's; all are reused here from the same
 * package. The statement lives in the section 2.1 chapter text.
 *
 * The scaffold returns the percentage tolerance of `a / a` paired with the
 * percentage tolerance of `a / b`, for `a` a 5 percent tolerance around 100
 * and `b` a 10 percent tolerance around 200.
 */
public fun par1(
    r1: Interval,
    r2: Interval,
): Interval = divInterval(mulInterval(r1, r2), addInterval(r1, r2))

public fun par2(
    r1: Interval,
    r2: Interval,
): Interval {
    val one = makeInterval(1.0, 1.0)
    return divInterval(one, addInterval(divInterval(one, r1), divInterval(one, r2)))
}

public fun ex_2_14(): Pair<Double, Double> = throw PendingExercise()
