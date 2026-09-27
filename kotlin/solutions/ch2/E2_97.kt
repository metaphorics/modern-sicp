// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.97

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.97: reduce rational functions to lowest terms. (a)
 * `reduceTerms` computes the GCD, integerizes both parts before dividing
 * by it, and strips the redundant content factors; `reducePoly` strips
 * and reattaches the variable. (b) `reduceIntegers` does for integers
 * what the original `makeRat` did, and the generic `reduce` dispatches to
 * either. The rational-function package's constructor then calls
 * `reduce`, so the whole system answers in lowest terms.
 *
 * The book's three-step reduction of `n / d` to lowest terms.
 */
context(r: Raise<GenError>)
public fun reduceTerms(
    table: NumTable,
    n: PersistentList<Term>,
    d: PersistentList<Term>,
): Pair<PersistentList<Term>, PersistentList<Term>> {
    val gRaw = gcdTerms96(table, n, d)
    val leadC = coeffOf(firstTerm(gRaw)) as? ZLong ?: r.raise(GenError.BadArgs("reduce", "non-integer leading coefficient"))
    // The Euclidean algorithm fixes the GCD only up to sign; normalize its
    // leading coefficient positive so num and den come out in the book's
    // conventional sign rather than both negated together.
    val g = if (leadC.n < 0) scaleTerms(table, gRaw, -1L) else gRaw
    val oN = orderOf(firstTerm(n))
    val oD = orderOf(firstTerm(d))
    val oG = orderOf(firstTerm(g))
    val factor = integerizeFactor(table, g, 1 + maxOf(oN, oD) - oG)
    val scaledN = scaleTerms(table, n, factor)
    val scaledD = scaleTerms(table, d, factor)
    val nn = divTerms(table, scaledN, g).first
    val dd = divTerms(table, scaledD, g).first
    val content = gcdAbs(coeffContent(table, nn), coeffContent(table, dd))
    val cleanedN = if (content > 1L) divideCoeffs(table, nn, content) else nn
    val cleanedD = if (content > 1L) divideCoeffs(table, dd, content) else dd
    return cleanedN to cleanedD
}

/** The leading coefficient of `g` to the `1 + O1 - O2` power. */
context(r: Raise<GenError>)
private fun integerizeFactor(
    table: NumTable,
    g: PersistentList<Term>,
    k: Int,
): Long {
    val c = coeffOf(firstTerm(g)) as? ZLong ?: r.raise(GenError.BadArgs("reduce", "non-integer leading coefficient"))
    return pseudoFactor(c.n, k)
}

private fun pseudoFactor(
    c: Long,
    k: Int,
): Long {
    var acc = 1L
    repeat(k) {
        acc = Math.multiplyExact(acc, c)
    }
    return acc
}

context(r: Raise<GenError>)
private fun scaleTerms(
    table: NumTable,
    l: PersistentList<Term>,
    k: Long,
): PersistentList<Term> = l.map { t -> makeTerm(orderOf(t), mul(table, coeffOf(t), ZLong(k))) }.toPersistentList()

/** `reduce-poly`: same variable in, reduced pair of polys out. */
context(r: Raise<GenError>)
public fun reducePoly(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Pair<Poly, Poly> {
    if (!sameVariable(p1.v, p2.v)) r.raise(GenError.NotSameVariable("reduce-poly", p1.v, p2.v))
    val (nn, dd) = reduceTerms(table, p1.terms, p2.terms)
    return Poly(p1.v, nn) to Poly(p1.v, dd)
}

/** The book's `reduce-integers`. */
public fun reduceIntegers(
    n: Long,
    d: Long,
): Pair<Long, Long> {
    val g = gcdAbs(n, d)
    return n / g to d / g
}

private fun gcdAbs(
    a: Long,
    b: Long,
): Long = if (b == 0L) kotlin.math.abs(a) else gcdAbs(b, a % b)

/** The generic `reduce`, dispatching on the argument types. */
context(r: Raise<GenError>)
public fun reduce(
    table: NumTable,
    n: Num,
    d: Num,
): Rat {
    val tags = listOf(typeTagOf(n), typeTagOf(d))
    return when (tags) {
        listOf("polynomial", "polynomial") -> {
            val (a, b) = reducePoly(table, n as Poly, d as Poly)
            Rat(a, b)
        }

        listOf("integer", "integer") -> {
            val (a, b) = reduceIntegers((n as ZLong).n, (d as ZLong).n)
            Rat(ZLong(a), ZLong(b))
        }

        else -> {
            r.raise(GenError.NoMethod("reduce", tags))
        }
    }
}

/** The reduced rational-function package: `makeRat` now reduces through
 * the generic operation, so every operation of the package answers in
 * lowest terms. */
public fun installReducedRationalFunctionPackage(table: NumTable) {
    installRationalFunctionPackageBase(table) { n, d ->
        arrow.core.raise
            .either { reduce(table, n, d) }
            .getOrNull() ?: Rat(n, d)
    }
}

/** Builds the book's `rf1` and `rf2`, adds them with the generic `add`,
 * and reports the reduced sum, which should read as
 * `(x^3 + 2x^2 + 3x + 1)/(x^4 + x^3 - x - 1)`. */
public fun ex_2_97(): String {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    installReducedRationalFunctionPackage(table)
    val p1 = makePolynomial("x", listOf(Term(1, ZLong(1)), Term(0, ZLong(1))))
    val p2 = makePolynomial("x", listOf(Term(3, ZLong(1)), Term(0, ZLong(-1))))
    val p3 = makePolynomial("x", listOf(Term(1, ZLong(1))))
    val p4 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(0, ZLong(-1))))
    return arrow.core.raise
        .either {
            val rf1 = reduce(table, p1, p2)
            val rf2 = reduce(table, p3, p4)
            val sum = applyGeneric(table, "add", listOf(rf1, rf2))
            show(sum)
        }.getOrNull() ?: "unreachable"
}
