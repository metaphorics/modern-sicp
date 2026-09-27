// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 90

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

private fun List<Term>.toTermList92(): PersistentList<Term> = fold(persistentListOf<Term>()) { acc, t -> acc.adding(t) }

private fun emptyTermList(): PersistentList<Term> = persistentListOf()

private fun List<Num>.toPersistentListD(): PersistentList<Num> = fold(persistentListOf<Num>()) { acc, n -> acc.adding(n) }

public class E2_90Test :
    FunSpec({
        test("both term-list providers agree on the book's sum and product") {
            ex_2_90() shouldBe true
        }

        test("the dense provider derives orders from positions") {
            val l = listOf(ZLong(1), ZLong(2), ZLong(3)).toPersistentListD()
            DenseTermsRep.first(l) shouldBe Term(2, ZLong(1))
            DenseTermsRep.first(DenseTermsRep.rest(l)) shouldBe Term(1, ZLong(2))
        }
    })
