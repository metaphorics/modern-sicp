// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.93a

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.93a is added by this edition and extends exercise 2.93; the
 * map's idea is "reduce rational functions lazily". Exercise 2.97 makes
 * every rational construction pay for the GCD immediately, even
 * when the caller only stacks fractions and never inspects the result.
 * [LazyRatF] defers the reduction: the unreduced numerator and
 * denominator are kept as given, and the reduced pair is computed once,
 * on first access, by Kotlin's `by lazy` delegate -- then memoized for
 * every later reader.
 *
 * A rational function whose reduction runs on first inspection, not at
 * construction.
 */
public class LazyRatF(
    /** The unreduced numerator. */
    public val num: Num,
    /** The unreduced denominator. */
    public val den: Num,
    /** The reduction to run once: it answers the reduced pair. */
    public val reduceFn: (Num, Num) -> Pair<Num, Num>,
) {
    /** The reduced pair: computed at most once, on the first access. */
    public val reduced: Pair<Num, Num> by lazy { reduceFn(num, den) }

    public override fun toString(): String {
        val (n, d) = reduced
        return "(${show(n)})/(${show(d)})"
    }
}

/** Adds two lazy rationals without reducing either operand: the sum's
 * parts are the book's cross-multiplication, still unreduced, still
 * cheap. */
context(r: Raise<GenError>)
public fun addLazy(
    table: NumTable,
    a: LazyRatF,
    b: LazyRatF,
    reduceFn: (Num, Num) -> Pair<Num, Num>,
): LazyRatF =
    LazyRatF(
        add(table, mul(table, a.num, b.den), mul(table, b.num, a.den)),
        mul(table, a.den, b.den),
        reduceFn,
    )

/** Builds a lazy rational function from polys and proves the laziness: a
 * reduction counter reads zero before any inspection, one after the
 * first, and still one after a second read -- the delegate memoized. */
public fun ex_2_93a(): Triple<Int, Int, Boolean> {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    installRationalFunctionPackage(table)
    var reductions = 0
    val countedReduce: (Num, Num) -> Pair<Num, Num> = { n, d ->
        reductions += 1
        arrow.core.raise
            .either { reduce(table, n, d) }
            .map { r -> r.num to r.den }
            .getOrNull() ?: (n to d)
    }
    val p1 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(0, ZLong(1))))
    val p2 = makePolynomial("x", listOf(Term(3, ZLong(1)), Term(0, ZLong(1))))
    val lazyRf = LazyRatF(p2, p1, countedReduce)
    val before = reductions
    val first = lazyRf.reduced
    val afterFirst = reductions
    val second = lazyRf.reduced
    val memoized = reductions == afterFirst && first == second
    return Triple(before, afterFirst, memoized)
}
