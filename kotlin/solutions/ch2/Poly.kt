// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.5.3: polynomials and generic-coefficient rationals

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/** One term: the order (power of the indeterminate) and a generic
 * coefficient. */
public data class Term(
    public val order: Int,
    public val coeff: Num,
)

/**
 * The book's `poly`: a variable and a sparse term list, highest order
 * first. The term list is `PersistentList<Term>`, the edition's list.
 */
public data class Poly(
    public val v: String,
    public val terms: PersistentList<Term>,
) : Num

/**
 * The generic-coefficient rational of exercise 2.93: numerator and
 * denominator are any two tower values the generic operations know --
 * integers or polynomials. The tower's own exact rational [QRat] is a
 * different level, so this package takes the tag `rational-function`.
 */
public data class Rat(
    public val num: Num,
    public val den: Num,
) : Num

/** The book's `same-variable?` of 2.3.2: variables are symbols, compared
 * by name. */
public fun sameVariable(
    a: String,
    b: String,
): Boolean = a == b

/** The book's `the-empty-termlist`. */
public fun theEmptyTermlist(): PersistentList<Term> = persistentListOf()

/** The book's `empty-termlist?`. */
public fun emptyTermlistQ(terms: PersistentList<Term>): Boolean = terms.isEmpty()

/** The book's `first-term`: the highest-order term. */
public fun firstTerm(terms: PersistentList<Term>): Term = terms.first()

/** The book's `rest-terms`: all but the highest-order term. */
public fun restTerms(terms: PersistentList<Term>): PersistentList<Term> = terms.removingAt(0)

/** The book's `make-term`. */
public fun makeTerm(
    order: Int,
    coeff: Num,
): Term = Term(order, coeff)

/** The book's `order` selector. */
public fun orderOf(t: Term): Int = t.order

/** The book's `coeff` selector. */
public fun coeffOf(t: Term): Num = t.coeff

/** The book's `adjoin-term`: a zero coefficient adjoins nothing, so the
 * sparse list never stores a zero term. `=zero?` is the generic one the
 * predicates installed, dispatching on the coefficient's own level. */
context(r: Raise<GenError>)
public fun adjoinTerm(
    table: NumTable,
    t: Term,
    terms: PersistentList<Term>,
): PersistentList<Term> = if (isZeroG(table, t.coeff)) terms else (persistentListOf(t) + terms).toPersistentList()

/** The book's `add-terms`: merge the two ordered lists, combining equal
 * orders through the generic `add`. */
context(r: Raise<GenError>)
public fun addTerms(
    table: NumTable,
    l1: PersistentList<Term>,
    l2: PersistentList<Term>,
): PersistentList<Term> =
    when {
        emptyTermlistQ(l1) -> {
            l2
        }

        emptyTermlistQ(l2) -> {
            l1
        }

        else -> {
            val t1 = firstTerm(l1)
            val t2 = firstTerm(l2)
            when {
                orderOf(t1) > orderOf(t2) -> {
                    adjoinTerm(table, t1, addTerms(table, restTerms(l1), l2))
                }

                orderOf(t1) < orderOf(t2) -> {
                    adjoinTerm(table, t2, addTerms(table, l1, restTerms(l2)))
                }

                else -> {
                    adjoinTerm(
                        table,
                        makeTerm(orderOf(t1), add(table, coeffOf(t1), coeffOf(t2))),
                        addTerms(table, restTerms(l1), restTerms(l2)),
                    )
                }
            }
        }
    }

/** The book's `mul-terms`: multiply each term of the first list by all
 * terms of the second, then sum the results. */
context(r: Raise<GenError>)
public fun mulTerms(
    table: NumTable,
    l1: PersistentList<Term>,
    l2: PersistentList<Term>,
): PersistentList<Term> =
    if (emptyTermlistQ(l1)) {
        theEmptyTermlist()
    } else {
        addTerms(
            table,
            mulTermByAllTerms(table, firstTerm(l1), l2),
            mulTerms(table, restTerms(l1), l2),
        )
    }

/** The book's `mul-term-by-all-terms`. */
context(r: Raise<GenError>)
public fun mulTermByAllTerms(
    table: NumTable,
    t1: Term,
    l: PersistentList<Term>,
): PersistentList<Term> =
    if (emptyTermlistQ(l)) {
        theEmptyTermlist()
    } else {
        val t2 = firstTerm(l)
        adjoinTerm(
            table,
            makeTerm(orderOf(t1) + orderOf(t2), mul(table, coeffOf(t1), coeffOf(t2))),
            mulTermByAllTerms(table, t1, restTerms(l)),
        )
    }

/** The book's `add-poly`: combine termwise under one variable. */
context(r: Raise<GenError>)
public fun addPoly(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Poly {
    if (!sameVariable(p1.v, p2.v)) r.raise(GenError.NotSameVariable("add-poly", p1.v, p2.v))
    return Poly(p1.v, addTerms(table, p1.terms, p2.terms))
}

/** The book's `mul-poly`. */
context(r: Raise<GenError>)
public fun mulPoly(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Poly {
    if (!sameVariable(p1.v, p2.v)) r.raise(GenError.NotSameVariable("mul-poly", p1.v, p2.v))
    return Poly(p1.v, mulTerms(table, p1.terms, p2.terms))
}

/**
 * The polynomial package of 2.5.3: `add-poly` and `mul-poly` installed as
 * the `add` and `mul` operations for type `polynomial`, so coefficients
 * combine through the same generic front-ends the numbers use. A
 * polynomial is assembled by [makePolynomial] directly, since its
 * variable is host data the tower does not carry.
 */
public fun installPolynomialPackage(table: NumTable) {
    table.put("add", listOf("polynomial", "polynomial")) { args ->
        val (p1, p2) = twoPolys(args)
        addPoly(table, p1, p2)
    }
    table.put("mul", listOf("polynomial", "polynomial")) { args ->
        val (p1, p2) = twoPolys(args)
        mulPoly(table, p1, p2)
    }
}

internal fun Raise<GenError>.twoPolys(args: List<Num>): Pair<Poly, Poly> {
    val (a, b) = twoNums("polynomial arithmetic", args)
    if (a !is Poly || b !is Poly) raise(GenError.BadArgs("polynomial arithmetic", "expected two polynomials"))
    return a to b
}

/** Assembles a polynomial from a variable and a term list, the book's
 * `make-poly` wrapped for users of the package. */
public fun makePolynomial(
    v: String,
    terms: List<Term>,
): Poly = Poly(v, terms.fold(persistentListOf()) { acc, t -> acc.adding(t) })
