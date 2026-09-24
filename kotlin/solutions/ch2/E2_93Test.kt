// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 93

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

private fun List<Term>.toTermList92(): PersistentList<Term> = fold(persistentListOf<Term>()) { acc, t -> acc.adding(t) }

private fun emptyTermList(): PersistentList<Term> = persistentListOf()

private fun List<Num>.toPersistentListD(): PersistentList<Num> = fold(persistentListOf<Num>()) { acc, n -> acc.adding(n) }

public class E2_93Test :
    FunSpec({
        test("adding the rational function to itself leaves it unreduced") {
            ex_2_93() shouldBe "(2*x^5 + 2*x^3 + 2*x^2 + 2 in x)/(1*x^4 + 2*x^2 + 1 in x)"
        }

        test("the package dispatches to the tower's generic operations") {
            val table = NumTable()
            installGenericArithmetic(table)
            installNeg(table)
            installPolynomialPackage(table)
            installPolyIsZero(table)
            installRationalFunctionPackage(table)
            val a =
                makeRationalFunction(
                    makePolynomial("x", listOf(Term(1, ZLong(1)))),
                    makePolynomial("x", listOf(Term(1, ZLong(2)))),
                )
            val b =
                makeRationalFunction(
                    makePolynomial("x", listOf(Term(1, ZLong(3)))),
                    makePolynomial("x", listOf(Term(1, ZLong(4)))),
                )
            val result = arrow.core.raise.either { applyGeneric(table, "mul", listOf(a, b)) }
            result.getOrNull() shouldBe
                Rat(
                    Poly("x", listOf(Term(2, ZLong(3))).toTermList92()),
                    Poly("x", listOf(Term(2, ZLong(8))).toTermList92()),
                )
        }
    })
