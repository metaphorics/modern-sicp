// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.96

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.96: pseudodivision. Before any polynomial division in the
 * GCD computation, multiply the dividend by the integerizing factor
 * `c^(1 + O1 - O2)`, where `c` is the divisor's leading coefficient, so
 * the division introduces no fractions; the remainder is the
 * pseudoremainder. Part (b) divides the answer's coefficients by their
 * integer GCD. The edition applies that content reduction after every
 * pseudoremainder, not only to the final answer, which keeps the `Long`
 * coefficients in range and lands on the same reduced result.
 *
 * The integer content of a term list: the GCD of all coefficients.
 */
context(r: Raise<GenError>)
public fun coeffContent(
    table: NumTable,
    l: PersistentList<Term>,
): Long =
    l.fold(0L) { acc, t ->
        val c = (coeffOf(t) as? ZLong)?.n ?: r.raise(GenError.BadArgs("content", "non-integer coefficient"))
        if (acc == 0L) kotlin.math.abs(c) else gcdOf(acc, c)
    }

private fun gcdOf(
    a: Long,
    b: Long,
): Long = if (b == 0L) kotlin.math.abs(a) else gcdOf(b, a % b)

/** Divides every coefficient by the integer `k`. */
context(r: Raise<GenError>)
public fun divideCoeffs(
    table: NumTable,
    l: PersistentList<Term>,
    k: Long,
): PersistentList<Term> = l.map { t -> makeTerm(orderOf(t), div(table, coeffOf(t), ZLong(k))) }.toPersistentList()

/** The book's `pseudoremainder-terms`: scale the dividend by the
 * integerizing factor, then take the remainder. */
context(r: Raise<GenError>)
public fun pseudoremainderTerms(
    table: NumTable,
    a: PersistentList<Term>,
    b: PersistentList<Term>,
): PersistentList<Term> {
    if (emptyTermlistQ(a)) return theEmptyTermlist()
    val o1 = orderOf(firstTerm(a))
    val o2 = orderOf(firstTerm(b))
    val c = coeffOf(firstTerm(b)) as? ZLong ?: r.raise(GenError.BadArgs("pseudoremainder", "non-integer divisor"))
    val factor = integerize(c.n, 1 + o1 - o2)
    val scaled =
        a.map { t -> makeTerm(orderOf(t), mul(table, coeffOf(t), ZLong(factor))) }.toPersistentList()
    return remainderTerms(table, scaled, b)
}

private fun integerize(
    c: Long,
    k: Int,
): Long {
    var acc = 1L
    repeat(k) {
        acc = Math.multiplyExact(acc, c)
    }
    return acc
}

/** `gcd-terms` of exercise 2.96: Euclid over pseudoremainders, each
 * remainder content-reduced so the coefficients stay small. */
context(r: Raise<GenError>)
public fun gcdTerms96(
    table: NumTable,
    a: PersistentList<Term>,
    b: PersistentList<Term>,
    budget: Int = 16,
): PersistentList<Term> {
    if (emptyTermlistQ(b)) return a
    if (budget <= 0) r.raise(GenError.BadArgs("gcd-terms", "budget exhausted"))
    val rem = pseudoremainderTerms(table, a, b)
    val content = coeffContent(table, rem)
    val reduced = if (content > 1L) divideCoeffs(table, rem, content) else rem
    return gcdTerms96(table, b, reduced, budget - 1)
}

/** Verifies the book's claim: `greatest-common-divisor` of `Q1` and `Q2`
 * now produces exactly `P1` with integer coefficients. */
public fun ex_2_96(): String {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    return arrow.core.raise
        .either {
            val g = gcdTerms96(table, makeQ1(table).terms, makeQ2(table).terms)
            show(Poly("x", g))
        }.getOrNull() ?: "unreachable"
}
