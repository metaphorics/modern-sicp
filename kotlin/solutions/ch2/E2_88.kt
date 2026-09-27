// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.88

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.88: extend the polynomial system with subtraction. The
 * book's hint -- define a generic negation operation first -- is
 * followed here over the whole tower: `neg` for each number level, then
 * `neg-poly` negating every coefficient, and subtraction as addition of
 * the negation.
 *
 * Installs `neg` for the number levels and for polynomials.
 */
public fun installNeg(table: NumTable) {
    table.put("neg", listOf("integer")) { args ->
        val z = args.single()
        if (z !is ZLong) raise(GenError.BadArgs("neg", "expected an integer"))
        ZLong(Math.negateExact(z.n))
    }
    table.put("neg", listOf("bigint")) { args ->
        val z = args.single()
        if (z !is BigZ) raise(GenError.BadArgs("neg", "expected a bigint"))
        BigZ(z.n.negate())
    }
    table.put("neg", listOf("rational")) { args ->
        val z = args.single()
        if (z !is QRat) raise(GenError.BadArgs("neg", "expected a rational"))
        QRat(z.num.negate(), z.den)
    }
    table.put("neg", listOf("real")) { args ->
        val z = args.single()
        if (z !is Real) raise(GenError.BadArgs("neg", "expected a real"))
        Real(-z.d)
    }
    table.put("neg", listOf("complex")) { args ->
        val z = args.single()
        if (z !is Complex) raise(GenError.BadArgs("neg", "expected a complex number"))
        Complex(Rect(-repRealPart(z.rep), -repImagPart(z.rep)))
    }
    table.put("neg", listOf("polynomial")) { args ->
        val z = args.single()
        if (z !is Poly) raise(GenError.BadArgs("neg", "expected a polynomial"))
        negPoly(table, z)
    }
}

/** Negates every coefficient of a polynomial, nested polys included. */
context(r: Raise<GenError>)
public fun negPoly(
    table: NumTable,
    p: Poly,
): Poly = Poly(p.v, p.terms.map { t -> Term(t.order, applyGeneric(table, "neg", listOf(t.coeff))) }.toPersistentList())

context(r: Raise<GenError>)
internal fun subTerms(
    table: NumTable,
    l1: PersistentList<Term>,
    l2: PersistentList<Term>,
): PersistentList<Term> = addTerms(table, l1, negateTerms(table, l2))

context(r: Raise<GenError>)
private fun negateTerms(
    table: NumTable,
    l: PersistentList<Term>,
): PersistentList<Term> = l.map { t -> Term(t.order, applyGeneric(table, "neg", listOf(t.coeff))) }.toPersistentList()

/** Subtraction of polys under one variable: add the negation. */
context(r: Raise<GenError>)
public fun subPoly(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Poly {
    if (!sameVariable(p1.v, p2.v)) r.raise(GenError.NotSameVariable("sub-poly", p1.v, p2.v))
    return Poly(p1.v, subTerms(table, p1.terms, p2.terms))
}

/** Installs polynomial subtraction as the generic `sub` for the type. */
public fun installPolySubtraction(table: NumTable) {
    table.put("sub", listOf("polynomial", "polynomial")) { args ->
        val (p1, p2) = twoPolys(args)
        subPoly(table, p1, p2)
    }
}

/** Computes the book-style check `(x^2 + 2x + 1) - (x^2 + x)`, returning
 * the printed result `1*x + 1`. */
public fun ex_2_88(): String {
    val table = NumTable()
    installGenericArithmetic(table)
    installPolynomialPackage(table)
    installNeg(table)
    installPolySubtraction(table)
    val p1 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(1, ZLong(2)), Term(0, ZLong(1))))
    val p2 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(1, ZLong(1))))
    return arrow.core.raise
        .either { show(subPoly(table, p1, p2)) }
        .getOrNull() ?: "unreachable"
}
