// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.94

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

/**
 * Exercise 2.94: using `divTerms`, implement `remainder-terms` and define
 * `gcd-terms` by Euclid's algorithm on term lists, then `gcd-poly`, and
 * install a generic `greatest-common-divisor` that reduces to `gcd-poly`
 * for polynomials and to ordinary `gcd` for integers. The book's test
 * pair works out to `x^2 - x` up to sign.
 *
 * The book's `remainder-terms`: the remainder half of `divTerms`.
 */
context(r: Raise<GenError>)
public fun remainderTerms(
    table: NumTable,
    a: PersistentList<Term>,
    b: PersistentList<Term>,
): PersistentList<Term> = divTerms(table, a, b).second

/** Euclid's algorithm over term lists, with a step budget that reports
 * the stalled divisions this edition's integer coefficients can produce
 * (the raw book version can cycle; exercise 2.95 observes exactly that). */
context(r: Raise<GenError>)
public fun gcdTerms(
    table: NumTable,
    a: PersistentList<Term>,
    b: PersistentList<Term>,
    budget: Int = 32,
): PersistentList<Term> {
    if (emptyTermlistQ(b)) return a
    if (budget <= 0) r.raise(GenError.BadArgs("gcd-terms", "no progress under integer coefficient division"))
    return gcdTerms(table, b, remainderTerms(table, a, b), budget - 1)
}

/** The polynomial GCD: same variable, then Euclid on the term lists. */
context(r: Raise<GenError>)
public fun gcdPoly(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Poly {
    if (!sameVariable(p1.v, p2.v)) r.raise(GenError.NotSameVariable("gcd-poly", p1.v, p2.v))
    return Poly(p1.v, gcdTerms(table, p1.terms, p2.terms))
}

private fun gcdLong(
    a: Long,
    b: Long,
): Long = if (b == 0L) kotlin.math.abs(a) else gcdLong(b, a % b)

/** Installs the generic `greatest-common-divisor` for polynomials and
 * integers. */
public fun installGreatestCommonDivisor(table: NumTable) {
    table.put("greatest-common-divisor", listOf("polynomial", "polynomial")) { args ->
        val (a, b) = twoPolys(args)
        gcdPoly(table, a, b)
    }
    table.put("greatest-common-divisor", listOf("integer", "integer")) { args ->
        val (a, b) = twoNums("greatest-common-divisor", args)
        if (a !is ZLong || b !is ZLong) raise(GenError.BadArgs("greatest-common-divisor", "expected two integers"))
        ZLong(gcdLong(a.n, b.n))
    }
}

/** Runs the book's test pair through the generic operation and reports
 * the printed GCD. */
public fun ex_2_94(): String {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    installGreatestCommonDivisor(table)
    val p1 = makePolynomial("x", listOf(Term(4, ZLong(1)), Term(3, ZLong(-1)), Term(2, ZLong(-2)), Term(1, ZLong(2))))
    val p2 = makePolynomial("x", listOf(Term(3, ZLong(1)), Term(1, ZLong(-1))))
    return arrow.core.raise
        .either {
            val g = applyGeneric(table, "greatest-common-divisor", listOf(p1, p2))
            show(g)
        }.getOrNull() ?: "unreachable"
}
