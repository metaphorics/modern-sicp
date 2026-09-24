// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.91

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.91: division of polynomials. `divTerms` completes the
 * book's skeleton: divide leading term by leading term, multiply, subtract,
 * recurse; when the divisor's order exceeds the dividend's, the dividend
 * is the remainder; when a leading division yields zero -- this edition's
 * integer coefficients truncate -- no progress is possible and the
 * dividend is the remainder. `divPoly` checks the variable, delegates,
 * and reattaches.
 *
 * The completed `div-terms`: quotient and remainder term lists.
 */
context(r: Raise<GenError>)
public fun divTerms(
    table: NumTable,
    l1: PersistentList<Term>,
    l2: PersistentList<Term>,
): Pair<PersistentList<Term>, PersistentList<Term>> {
    if (emptyTermlistQ(l1)) return theEmptyTermlist() to theEmptyTermlist()
    val t1 = firstTerm(l1)
    val t2 = firstTerm(l2)
    if (orderOf(t2) > orderOf(t1)) return theEmptyTermlist() to l1
    val newC = div(table, coeffOf(t1), coeffOf(t2))
    val newO = orderOf(t1) - orderOf(t2)
    if (isZeroG(table, newC)) return theEmptyTermlist() to l1
    val factor = persistentListOf(makeTerm(newO, newC))
    val product = mulTermByAllTerms(table, factor.first(), l2)
    val difference = addTerms(table, l1, negateAll(table, product))
    val (q, r) = divTerms(table, difference, l2)
    return (persistentListOf(makeTerm(newO, newC)) + q).toPersistentList() to r
}

context(r: Raise<GenError>)
private fun negateAll(
    table: NumTable,
    l: PersistentList<Term>,
): PersistentList<Term> =
    l
        .map { t -> makeTerm(orderOf(t), applyGeneric(table, "neg", listOf(t.coeff))) }
        .fold(persistentListOf()) { acc, t -> acc.adding(t) }

/** Divides two polys under one variable, answering quotient and
 * remainder polys. */
context(r: Raise<GenError>)
public fun divPoly(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Pair<Poly, Poly> {
    if (!sameVariable(p1.v, p2.v)) r.raise(GenError.NotSameVariable("div-poly", p1.v, p2.v))
    val (q, r) = divTerms(table, p1.terms, p2.terms)
    return Poly(p1.v, q) to Poly(p1.v, r)
}

/** Runs the book's example: `(x^5 - 1) / (x^2 - 1)` is `x^3 + x` with
 * remainder `x - 1`. */
public fun ex_2_91(): Pair<String, String> {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    val p1 = makePolynomial("x", listOf(Term(5, ZLong(1)), Term(0, ZLong(-1))))
    val p2 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(0, ZLong(-1))))
    return arrow.core.raise
        .either {
            val (q, r) = divPoly(table, p1, p2)
            show(q) to show(r)
        }.getOrNull() ?: ("unreachable" to "unreachable")
}
