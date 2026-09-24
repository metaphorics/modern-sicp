// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.89

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.89: implement the dense term-list representation -- a list
 * of coefficients, the order of a term being the length of the sublist
 * beginning with its coefficient, decremented by one. [DensePoly] keeps
 * the coefficients in a `PersistentList`, highest order first, and the
 * dense package performs addition and multiplication coefficient-wise
 * through the generic operations.
 *
 * A dense polynomial: one coefficient slot per order, highest first.
 */
public data class DensePoly(
    public val v: String,
    public val coeffs: PersistentList<Num>,
)

/** The dense term list constructors and selectors, the order read off
 * the position. */
public object DenseTerms {
    /** The empty dense term list. */
    public fun empty(): PersistentList<Num> = persistentListOf()

    /** Adjoins the term of the next higher order in front. */
    public fun adjoin(
        t: Term,
        l: PersistentList<Num>,
    ): PersistentList<Num> = (persistentListOf(t.coeff) + l).toPersistentList()

    /** The highest-order term. */
    public fun first(l: PersistentList<Num>): Term = Term(l.size - 1, l.first())

    /** All but the highest-order term. */
    public fun rest(l: PersistentList<Num>): PersistentList<Num> = l.removingAt(0)

    /** Emptiness. */
    public fun isEmpty(l: PersistentList<Num>): Boolean = l.isEmpty()
}

/** Adds two dense term lists coefficient-wise, filling the shorter with
 * zeros so the orders align. */
context(r: Raise<GenError>)
public fun addDenseTerms(
    table: NumTable,
    a: PersistentList<Num>,
    b: PersistentList<Num>,
): PersistentList<Num> {
    val width = maxOf(a.size, b.size)
    val padA = zeroPad(table, a, width)
    val padB = zeroPad(table, b, width)
    val sums = padA.zip(padB) { x, y -> add(table, x, y) }
    return trimLeadingZeros(table, sums)
}

context(r: Raise<GenError>)
private fun zeroPad(
    table: NumTable,
    l: PersistentList<Num>,
    width: Int,
): List<Num> = List(width) { i -> if (i < width - l.size) ZLong(0) else l[i - (width - l.size)] }

context(r: Raise<GenError>)
private fun trimLeadingZeros(
    table: NumTable,
    l: List<Num>,
): PersistentList<Num> {
    val firstNonzero = l.indexOfFirst { c -> !isZeroG(table, c) }
    val start = if (firstNonzero == -1) l.size - 1 else firstNonzero
    return l.drop(start).toPersistentList()
}

/** Multiplies two dense term lists: every term of one against every term
 * of the other, the products summed by [addDenseTerms]. */
context(r: Raise<GenError>)
public fun mulDenseTerms(
    table: NumTable,
    a: PersistentList<Num>,
    b: PersistentList<Num>,
): PersistentList<Num> =
    a.foldIndexed(persistentListOf()) { i, acc, c ->
        if (isZeroG(table, c)) {
            acc
        } else {
            val shifted = b.map { coeff -> mul(table, c, coeff) }
            val padded = shifted + List(a.size - 1 - i) { ZLong(0) }
            addDenseTerms(table, acc, padded.toPersistentList())
        }
    }

/** Assembles a dense polynomial from coefficients, highest order first. */
public fun makeDensePolynomial(
    v: String,
    coeffs: List<Num>,
): DensePoly = DensePoly(v, coeffs.fold(persistentListOf()) { acc, n -> acc.adding(n) })

/** Converts a dense coefficient list to the sparse term list it denotes,
 * dropping the zero coefficients dense keeps explicit and sparse never
 * stores -- the fair way to compare the two representations' results. */
context(r: Raise<GenError>)
private fun denseToSparseTerms(
    table: NumTable,
    coeffs: PersistentList<Num>,
): List<Term> {
    val top = coeffs.size - 1
    return coeffs.mapIndexedNotNull { i, c -> if (isZeroG(table, c)) null else Term(top - i, c) }
}

/** Proves the dense representation against the sparse one: the same two
 * book polynomials sum and multiply to the same coefficients either way.
 * Polynomial A has an internal zero at order 3, so dense's sum keeps an
 * explicit zero there while sparse never lists a coefficient it lacks --
 * both sides compare as term lists, not raw coefficient lists, so that
 * representational difference does not register as disagreement. */
public fun ex_2_89(): Boolean {
    val table = NumTable()
    installGenericArithmetic(table)
    installPolynomialPackage(table)
    val sparseA =
        makePolynomial("x", listOf(Term(5, ZLong(1)), Term(4, ZLong(2)), Term(2, ZLong(3)), Term(1, ZLong(-2)), Term(0, ZLong(-5))))
    val sparseB = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(0, ZLong(3))))
    val denseA = makeDensePolynomial("x", listOf(ZLong(1), ZLong(2), ZLong(0), ZLong(3), ZLong(-2), ZLong(-5)))
    val denseB = makeDensePolynomial("x", listOf(ZLong(1), ZLong(0), ZLong(3)))
    return arrow.core.raise
        .either {
            val sparseSum = addPoly(table, sparseA, sparseB).terms
            val denseSum = denseToSparseTerms(table, addDenseTerms(table, denseA.coeffs, denseB.coeffs))
            val sparseProduct = mulPoly(table, sparseA, sparseB).terms
            val denseProduct = denseToSparseTerms(table, mulDenseTerms(table, denseA.coeffs, denseB.coeffs))
            sparseSum.toList() == denseSum && sparseProduct.toList() == denseProduct
        }.getOrNull() ?: false
}
