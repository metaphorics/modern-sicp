// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.59

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_59Test :
    FunSpec({
        test("unionSet keeps every element of set1 not in set2, then all of set2") {
            unionSet(listOf(1L, 2L, 3L), listOf(2L, 3L, 4L)) shouldBe listOf(1L, 2L, 3L, 4L)
        }
        test("unionSet of disjoint sets keeps every element of both") {
            unionSet(listOf(1L, 2L), listOf(3L, 4L)) shouldBe listOf(1L, 2L, 3L, 4L)
        }
        test("unionSet with an empty set returns the other set") {
            unionSet(emptyList(), listOf(1L, 2L)) shouldBe listOf(1L, 2L)
            unionSet(listOf(1L, 2L), emptyList()) shouldBe listOf(1L, 2L)
        }
        test("every element of set1 and set2 is a member of the union") {
            val set1 = listOf(5L, 2L, 8L)
            val set2 = listOf(8L, 9L)
            val union = unionSet(set1, set2)
            (set1 + set2).forEach { elementOfSet(it, union) shouldBe true }
        }
        test("ex_2_59 unions {1,2,3} and {2,3,4}") {
            ex_2_59() shouldBe listOf(1L, 2L, 3L, 4L)
        }
    })
