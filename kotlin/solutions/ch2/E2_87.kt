// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.87

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.persistentListOf

/**
 * Extend the tower's zero predicate to polynomials, recursively testing every
 * coefficient, including coefficients that are themselves polynomials.
 */
public fun installPolyIsZero(table: NumTable) {
    table.put("=zero?", listOf("polynomial")) { args ->
        val z = args.single()
        if (z !is Poly) raise(GenError.BadArgs("=zero?", "expected a polynomial"))
        ZLong(if (polyIsZero(table, z)) 1 else 0)
    }
}

context(r: Raise<GenError>)
internal fun polyIsZero(
    table: NumTable,
    p: Poly,
): Boolean = p.terms.all { isZeroG(table, it.coeff) }

/** Proves the install: a polynomial zero by its plain coefficients, one
 * nonzero by a nested coefficient, and `adjoin-term` dropping a nested
 * zero-polynomial coefficient. */
public fun ex_2_87(): Boolean {
    val table = NumTable()
    installGenericArithmetic(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    val zeroY = Poly("y", persistentListOf())
    return arrow.core.raise
        .either {
            val flat = isZeroG(table, makePolynomial("x", listOf(Term(2, ZLong(0)), Term(0, ZLong(0)))))
            val nestedZero =
                isZeroG(
                    table,
                    makePolynomial(
                        "x",
                        listOf(Term(1, zeroY), Term(0, Poly("y", persistentListOf(Term(1, ZLong(0)))))),
                    ),
                )
            val nestedNonzero =
                !isZeroG(
                    table,
                    makePolynomial("x", listOf(Term(1, Poly("y", persistentListOf(Term(1, ZLong(1))))))),
                )
            val dropped = adjoinTerm(table, Term(2, zeroY), persistentListOf(Term(1, ZLong(3))))
            flat && nestedZero && nestedNonzero && dropped.size == 1
        }.getOrNull() ?: false
}
