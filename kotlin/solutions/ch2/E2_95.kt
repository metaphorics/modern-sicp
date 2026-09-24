// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.95

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.95: with `P1 = x^2 - 2x + 1`, `P2 = 11x^2 + 7`, and
 * `P3 = 13x + 5`, let `Q1 = P1*P2` and `Q2 = P1*P3` and compute their
 * GCD. The answer is not `P1`: `Q2`'s leading coefficient 13 does not
 * divide `Q1`'s leading coefficient 11, and this edition's integer
 * coefficient division truncates, so the first quotient term comes out
 * zero, no progress is possible, and Euclid's algorithm cannot descend.
 * [traceGcdTerms] records each step so the failure stays observable --
 * exactly the "try tracing gcd-terms" the exercise asks for.
 *
 * The book's three polynomials and their two products.
 */
context(r: Raise<GenError>)
public fun makeQ1(table: NumTable): Poly =
    mulPoly(
        table,
        makePolynomial("x", listOf(Term(2, ZLong(1)), Term(1, ZLong(-2)), Term(0, ZLong(1)))),
        makePolynomial("x", listOf(Term(2, ZLong(11)), Term(0, ZLong(7)))),
    )

context(r: Raise<GenError>)
public fun makeQ2(table: NumTable): Poly =
    mulPoly(
        table,
        makePolynomial("x", listOf(Term(2, ZLong(1)), Term(1, ZLong(-2)), Term(0, ZLong(1)))),
        makePolynomial("x", listOf(Term(1, ZLong(13)), Term(0, ZLong(5)))),
    )

/** One trace step: the division Euclid attempted and what it produced. */
public data class GcdStep(
    public val dividend: String,
    public val divisor: String,
    public val remainder: String,
)

context(r: Raise<GenError>)
private fun gcdTermsTraced(
    table: NumTable,
    a: PersistentList<Term>,
    b: PersistentList<Term>,
    steps: MutableList<GcdStep>,
    budget: Int,
): PersistentList<Term> {
    if (emptyTermlistQ(b)) return a
    if (budget <= 0) r.raise(GenError.BadArgs("gcd-terms", "budget exhausted"))
    val rem = remainderTerms(table, a, b)
    steps.add(GcdStep(show(Poly("x", a)), show(Poly("x", b)), show(Poly("x", rem))))
    return gcdTermsTraced(table, b, rem, steps, budget - 1)
}

/** Traces Euclid on `Q1` and `Q2` and reports whether the trace shows a
 * stalled division (remainder equals the dividend, so the next round
 * only swaps the operands) and whether any answer at all emerged. */
public fun ex_2_95(): List<String> {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    val steps = mutableListOf<GcdStep>()
    val outcome = arrow.core.raise.either { gcdTermsTraced(table, makeQ1(table).terms, makeQ2(table).terms, steps, 8) }
    if (steps.isEmpty()) return listOf("no-steps")
    val stalled = steps.any { it.remainder == it.dividend }
    val gcdAnswer = outcome.getOrNull()?.let { g -> show(Poly("x", g)) } ?: "none"
    val p1 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(1, ZLong(-2)), Term(0, ZLong(1))))
    val notP1 = gcdAnswer != show(p1)
    return listOf(
        "stalled=$stalled",
        "not-p1=$notP1",
        "gcd=$gcdAnswer",
        "first-remainder=${steps.first().remainder}",
    )
}
