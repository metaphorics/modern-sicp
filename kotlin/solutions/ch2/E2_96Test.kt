// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 96

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

private fun List<Term>.toTermList92(): PersistentList<Term> = fold(persistentListOf<Term>()) { acc, t -> acc.adding(t) }

private fun emptyTermList(): PersistentList<Term> = persistentListOf()

private fun List<Num>.toPersistentListD(): PersistentList<Num> = fold(persistentListOf<Num>()) { acc, n -> acc.adding(n) }

public class E2_96Test :
    FunSpec({
        test("the pseudoremainder GCD of Q1 and Q2 is exactly P1") {
            ex_2_96() shouldBe "1*x^2 + (-2)*x + 1 in x"
        }

        test("pseudoremainder keeps coefficients integral on the book's example") {
            val table = NumTable()
            installGenericArithmetic(table)
            installNeg(table)
            installPolynomialPackage(table)
            installPolyIsZero(table)
            val rem =
                arrow.core.raise.either {
                    pseudoremainderTerms(table, makeQ1(table).terms, makeQ2(table).terms)
                }
            rem.getOrNull() shouldBe
                listOf(
                    Term(2, ZLong(1458)),
                    Term(1, ZLong(-2916)),
                    Term(0, ZLong(1458)),
                ).toTermList92()
        }
    })
