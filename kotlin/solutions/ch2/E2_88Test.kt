// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 88

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

private fun List<Term>.toTermList92(): PersistentList<Term> = fold(persistentListOf<Term>()) { acc, t -> acc.adding(t) }

private fun emptyTermList(): PersistentList<Term> = persistentListOf()

private fun List<Num>.toPersistentListD(): PersistentList<Num> = fold(persistentListOf<Num>()) { acc, n -> acc.adding(n) }

public class E2_88Test :
    FunSpec({
        test("(x^2 + 2x + 1) - (x^2 + x) leaves x + 1") {
            ex_2_88() shouldBe "1*x + 1 in x"
        }

        test("negation of a nested polynomial negates coefficients") {
            val table = NumTable()
            installGenericArithmetic(table)
            installPolynomialPackage(table)
            installNeg(table)
            installPolyIsZero(table)
            val p =
                makePolynomial(
                    "x",
                    listOf(Term(1, Poly("y", listOf(Term(1, ZLong(2))).toTermList92()))),
                )
            val result = arrow.core.raise.either { show(negPoly(table, p)) }
            result.getOrNull() shouldBe "(-2)*y in y*x in x"
        }
    })
