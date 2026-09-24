// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 91

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

private fun List<Term>.toTermList92(): PersistentList<Term> = fold(persistentListOf<Term>()) { acc, t -> acc.adding(t) }

private fun emptyTermList(): PersistentList<Term> = persistentListOf()

private fun List<Num>.toPersistentListD(): PersistentList<Num> = fold(persistentListOf<Num>()) { acc, n -> acc.adding(n) }

public class E2_91Test :
    FunSpec({
        test("the book's division gives x^3 + x remainder x - 1") {
            ex_2_91() shouldBe ("1*x^3 + 1*x in x" to "1*x + (-1) in x")
        }

        test("a lower-order dividend is its own remainder with an empty quotient") {
            val table = NumTable()
            installGenericArithmetic(table)
            installNeg(table)
            installPolynomialPackage(table)
            installPolyIsZero(table)
            val p1 = makePolynomial("x", listOf(Term(1, ZLong(1))))
            val p2 = makePolynomial("x", listOf(Term(2, ZLong(1))))
            val result = arrow.core.raise.either { divPoly(table, p1, p2) }
            result.getOrNull() shouldBe (Poly("x", emptyTermList()) to Poly("x", listOf(Term(1, ZLong(1))).toTermList92()))
        }
    })
