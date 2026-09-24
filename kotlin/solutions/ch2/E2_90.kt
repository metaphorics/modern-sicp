// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.90

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * Exercise 2.90: a polynomial system efficient for both sparse and dense
 * polynomials. The two term-list representations sit behind one interface
 * -- [TermListRepresentation] -- and the term-list algorithms
 * [addTermsVia]/[mulTermsVia] are written once against it. The sparse
 * provider stores `(order coeff)` terms; the dense provider stores plain
 * coefficients and derives each order from the position, exactly the two
 * representations the section contrasts.
 *
 * The interface both term-list providers answer to.
 */
public interface TermListRepresentation<L> {
    /** The empty list. */
    public fun empty(): L

    /** Emptiness. */
    public fun isEmpty(l: L): Boolean

    /** The highest-order term. */
    public fun first(l: L): Term

    /** All but the highest-order term. */
    public fun rest(l: L): L

    /** Adjoins a term of strictly higher order in front. */
    public fun adjoin(
        t: Term,
        l: L,
    ): L
}

/** The sparse provider: a list of `(order coeff)` terms. */
public object SparseTermsRep : TermListRepresentation<PersistentList<Term>> {
    override fun empty(): PersistentList<Term> = persistentListOf()

    override fun isEmpty(l: PersistentList<Term>): Boolean = l.isEmpty()

    override fun first(l: PersistentList<Term>): Term = l.first()

    override fun rest(l: PersistentList<Term>): PersistentList<Term> = l.removingAt(0)

    override fun adjoin(
        t: Term,
        l: PersistentList<Term>,
    ): PersistentList<Term> = (persistentListOf(t) + l).toPersistentList()
}

/** The dense provider: plain coefficients, order = size - position - 1. */
public object DenseTermsRep : TermListRepresentation<PersistentList<Num>> {
    override fun empty(): PersistentList<Num> = persistentListOf()

    override fun isEmpty(l: PersistentList<Num>): Boolean = l.isEmpty()

    override fun first(l: PersistentList<Num>): Term = Term(l.size - 1, l.first())

    override fun rest(l: PersistentList<Num>): PersistentList<Num> = l.removingAt(0)

    /** Adjoins `t`, padding any gap above the current top with zero
     * coefficients so a caller may adjoin non-contiguous orders, as
     * `mul-term-by-all-terms` does when it builds a product term by term. */
    override fun adjoin(
        t: Term,
        l: PersistentList<Num>,
    ): PersistentList<Num> {
        require(t.order >= l.size) { "dense adjoin expects an order at or above the current top" }
        val gap = List(t.order - l.size) { ZLong(0) }
        return (persistentListOf(t.coeff) + gap + l).toPersistentList()
    }
}

/** `add-terms`, written once against the provider interface. */
context(r: Raise<GenError>)
public fun <L> addTermsVia(
    rep: TermListRepresentation<L>,
    table: NumTable,
    l1: L,
    l2: L,
): L =
    when {
        rep.isEmpty(l1) -> {
            l2
        }

        rep.isEmpty(l2) -> {
            l1
        }

        else -> {
            val t1 = rep.first(l1)
            val t2 = rep.first(l2)
            when {
                orderOf(t1) > orderOf(t2) -> {
                    rep.adjoin(t1, addTermsVia(rep, table, rep.rest(l1), l2))
                }

                orderOf(t1) < orderOf(t2) -> {
                    rep.adjoin(t2, addTermsVia(rep, table, l1, rep.rest(l2)))
                }

                else -> {
                    rep.adjoin(
                        makeTerm(orderOf(t1), add(table, coeffOf(t1), coeffOf(t2))),
                        addTermsVia(rep, table, rep.rest(l1), rep.rest(l2)),
                    )
                }
            }
        }
    }

/** `mul-term-by-all-terms`, against the provider interface. */
context(r: Raise<GenError>)
public fun <L> mulTermByAllTermsVia(
    rep: TermListRepresentation<L>,
    table: NumTable,
    t1: Term,
    l: L,
): L =
    if (rep.isEmpty(l)) {
        rep.empty()
    } else {
        val t2 = rep.first(l)
        rep.adjoin(
            makeTerm(orderOf(t1) + orderOf(t2), mul(table, coeffOf(t1), coeffOf(t2))),
            mulTermByAllTermsVia(rep, table, t1, rep.rest(l)),
        )
    }

/** `mul-terms`, against the provider interface. */
context(r: Raise<GenError>)
public fun <L> mulTermsVia(
    rep: TermListRepresentation<L>,
    table: NumTable,
    l1: L,
    l2: L,
): L =
    if (rep.isEmpty(l1)) {
        rep.empty()
    } else {
        addTermsVia(
            rep,
            table,
            mulTermByAllTermsVia(rep, table, rep.first(l1), l2),
            mulTermsVia(rep, table, rep.rest(l1), l2),
        )
    }

/** Projects any provider's term list onto the sparse form, so results of
 * the two representations can be compared: zero coefficients are
 * dropped, since dense keeps an explicit zero for every unoccupied
 * order while sparse never stores one, a representational difference
 * that must not register as the providers disagreeing. */
context(r: Raise<GenError>)
public fun <L> toSparseTerms(
    table: NumTable,
    rep: TermListRepresentation<L>,
    l: L,
): PersistentList<Term> =
    if (rep.isEmpty(l)) {
        persistentListOf()
    } else {
        val t = rep.first(l)
        val rest = toSparseTerms(table, rep, rep.rest(l))
        if (isZeroG(table, coeffOf(t))) rest else (persistentListOf(t) + rest).toPersistentList()
    }

/** Runs the same book sum and product through both providers and checks
 * the results agree in sparse form. */
public fun ex_2_90(): Boolean {
    val table = NumTable()
    installGenericArithmetic(table)
    val sparseA = listOf(Term(5, ZLong(1)), Term(4, ZLong(2)), Term(2, ZLong(3)), Term(1, ZLong(-2)), Term(0, ZLong(-5))).toPersistentList()
    val sparseB = listOf(Term(2, ZLong(1)), Term(0, ZLong(3))).toPersistentList()
    val denseA = listOf(ZLong(1), ZLong(2), ZLong(0), ZLong(3), ZLong(-2), ZLong(-5)).toPersistentList()
    val denseB = listOf(ZLong(1), ZLong(0), ZLong(3)).toPersistentList()
    return arrow.core.raise
        .either {
            val sumSparse = toSparseTerms(table, SparseTermsRep, addTermsVia(SparseTermsRep, table, sparseA, sparseB))
            val sumDense = toSparseTerms(table, DenseTermsRep, addTermsVia(DenseTermsRep, table, denseA, denseB))
            val mulSparse = toSparseTerms(table, SparseTermsRep, mulTermsVia(SparseTermsRep, table, sparseA, sparseB))
            val mulDense = toSparseTerms(table, DenseTermsRep, mulTermsVia(DenseTermsRep, table, denseA, denseB))
            sumSparse == sumDense && mulSparse == mulDense
        }.getOrNull() ?: false
}
