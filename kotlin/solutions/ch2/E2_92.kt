// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.92

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

/**
 * Exercise 2.92: impose an ordering on variables so polynomial addition
 * and multiplication work for polynomials in different variables. The
 * dominant variable is canonical: a polynomial in a lower variable lifts
 * into the dominant one as a single order-zero term, so no expansion
 * happens. Coefficient arithmetic learns one new move alongside -- an
 * integer constant meets a polynomial by becoming a constant polynomial
 * in the same variable, the coercion the section's footnote asks for.
 *
 * The variable ordering: a higher rank dominates.
 */
public val variableRank: Map<String, Int> = mapOf("x" to 3, "y" to 2, "z" to 1)

private fun rankOf(v: String): Int = variableRank[v] ?: 0

/** The dominant variable of two. */
public fun dominantVariable(
    a: String,
    b: String,
): String = if (rankOf(a) >= rankOf(b)) a else b

/** Lifts `p` into the variable `v`: itself, a constant term, or a
 * refusal -- converting toward a lower variable would expand the poly. */
context(r: Raise<GenError>)
public fun rebase92(
    p: Poly,
    v: String,
): Poly =
    when {
        p.v == v -> p
        rankOf(v) > rankOf(p.v) -> Poly(v, persistentListOf(Term(0, p)))
        else -> r.raise(GenError.BadArgs("rebase", "cannot lower ${p.v} into $v without expanding"))
    }

/** Coefficient addition under the ordering: a constant meets a poly by
 * becoming a constant poly in the same variable. */
context(r: Raise<GenError>)
public fun addCoeffs92(
    table: NumTable,
    a: Num,
    b: Num,
): Num =
    when {
        a is Poly && b is ZLong -> Poly(a.v, persistentListOf(Term(0, b)))
        a is ZLong && b is Poly -> Poly(b.v, persistentListOf(Term(0, a)))
        a is Poly && b is Poly -> addPoly92(table, a, b)
        else -> add(table, a, b)
    }

/** Coefficient multiplication under the ordering: a constant scales a
 * poly term by term. */
context(r: Raise<GenError>)
public fun mulCoeffs92(
    table: NumTable,
    a: Num,
    b: Num,
): Num =
    when {
        a is Poly && b is ZLong -> scalePoly92(table, a, b)
        a is ZLong && b is Poly -> scalePoly92(table, b, a)
        a is Poly && b is Poly -> mulPoly92(table, a, b)
        else -> mul(table, a, b)
    }

context(r: Raise<GenError>)
private fun scalePoly92(
    table: NumTable,
    p: Poly,
    k: ZLong,
): Poly {
    val scaled = p.terms.map { t -> Term(t.order, mulCoeffs92(table, t.coeff, k)) }
    return Poly(p.v, scaled.fold(persistentListOf()) { acc, t -> acc.adding(t) })
}

context(r: Raise<GenError>)
private fun addTerms92(
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
                    adjoinTerm(table, t1, addTerms92(table, restTerms(l1), l2))
                }

                orderOf(t1) < orderOf(t2) -> {
                    adjoinTerm(table, t2, addTerms92(table, l1, restTerms(l2)))
                }

                else -> {
                    adjoinTerm(
                        table,
                        makeTerm(orderOf(t1), addCoeffs92(table, coeffOf(t1), coeffOf(t2))),
                        addTerms92(table, restTerms(l1), restTerms(l2)),
                    )
                }
            }
        }
    }

context(r: Raise<GenError>)
private fun mulTermByAllTerms92(
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
            makeTerm(orderOf(t1) + orderOf(t2), mulCoeffs92(table, coeffOf(t1), coeffOf(t2))),
            mulTermByAllTerms92(table, t1, restTerms(l)),
        )
    }

context(r: Raise<GenError>)
private fun mulTerms92(
    table: NumTable,
    l1: PersistentList<Term>,
    l2: PersistentList<Term>,
): PersistentList<Term> =
    if (emptyTermlistQ(l1)) {
        theEmptyTermlist()
    } else {
        addTerms92(
            table,
            mulTermByAllTerms92(table, firstTerm(l1), l2),
            mulTerms92(table, restTerms(l1), l2),
        )
    }

/** Addition under the variable ordering. */
context(r: Raise<GenError>)
public fun addPoly92(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Poly {
    val v = dominantVariable(p1.v, p2.v)
    return Poly(v, addTerms92(table, rebase92(p1, v).terms, rebase92(p2, v).terms))
}

/** Multiplication under the variable ordering. */
context(r: Raise<GenError>)
public fun mulPoly92(
    table: NumTable,
    p1: Poly,
    p2: Poly,
): Poly {
    val v = dominantVariable(p1.v, p2.v)
    return Poly(v, mulTerms92(table, rebase92(p1, v).terms, rebase92(p2, v).terms))
}

/** Runs the book's mixed-variable checks: the product's `x^3`
 * coefficient of `[(y+1)x^2 + (y^2+1)x + (y-1)] * [(y-2)x + (y^3+7)]`
 * comes out `y^2 - y - 2`, and a poly in `y` adds into a poly in `x` as
 * its constant term. */
public fun ex_2_92(): Pair<Boolean, Boolean> {
    val table = NumTable()
    installGenericArithmetic(table)
    installPolynomialPackage(table)
    installNeg(table)
    installPolyIsZero(table)
    val p1 =
        makePolynomial(
            "x",
            listOf(
                Term(2, Poly("y", listOf(Term(1, ZLong(1)), Term(0, ZLong(1))).toTermList())),
                Term(1, Poly("y", listOf(Term(2, ZLong(1)), Term(0, ZLong(1))).toTermList())),
                Term(0, Poly("y", listOf(Term(1, ZLong(1)), Term(0, ZLong(-1))).toTermList())),
            ),
        )
    val p2 =
        makePolynomial(
            "x",
            listOf(
                Term(1, Poly("y", listOf(Term(1, ZLong(1)), Term(0, ZLong(-2))).toTermList())),
                Term(0, Poly("y", listOf(Term(3, ZLong(1)), Term(0, ZLong(7))).toTermList())),
            ),
        )
    val inX = makePolynomial("x", listOf(Term(2, ZLong(1))))
    val inY = makePolynomial("y", listOf(Term(1, ZLong(1))))
    return arrow.core.raise
        .either {
            val product = mulPoly92(table, p1, p2)
            val cubic = product.terms.first { it.order == 3 }.coeff
            val mixed = addPoly92(table, inX, inY)
            val cubicOk =
                cubic == Poly("y", listOf(Term(2, ZLong(1)), Term(1, ZLong(-1)), Term(0, ZLong(-2))).toTermList())
            val mixedOk =
                mixed ==
                    Poly(
                        "x",
                        listOf(Term(2, ZLong(1)), Term(0, Poly("y", listOf(Term(1, ZLong(1))).toTermList()))).toTermList(),
                    )
            cubicOk to mixedOk
        }.getOrNull() ?: (false to false)
}

private fun List<Term>.toTermList(): PersistentList<Term> = fold(persistentListOf()) { acc, t -> acc.adding(t) }
