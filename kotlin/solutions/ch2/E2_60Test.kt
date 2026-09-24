// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.60

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_60Test :
    FunSpec({
        test("elementOfSetDup finds a duplicated element") {
            elementOfSetDup(2L, listOf(2L, 3L, 2L, 1L)) shouldBe true
            elementOfSetDup(9L, listOf(2L, 3L, 2L, 1L)) shouldBe false
        }
        test("adjoinSetDup always prepends, growing the list even for an already-present element") {
            adjoinSetDup(2L, listOf(2L, 3L)) shouldBe listOf(2L, 2L, 3L)
        }
        test("unionSetDup is a plain concatenation, duplicates and all") {
            unionSetDup(listOf(2L, 3L, 2L), listOf(3L, 4L)) shouldBe listOf(2L, 3L, 2L, 3L, 4L)
        }
        test("intersectionSetDup keeps a duplicated element once per occurrence in set1") {
            intersectionSetDup(listOf(2L, 3L, 2L, 1L), listOf(1L, 2L)) shouldBe listOf(2L, 2L, 1L)
        }
        test("ex_2_60 unions the book's dup-encoded {1,2,3} with {1,3}") {
            ex_2_60() shouldBe listOf(2L, 3L, 2L, 1L, 3L, 2L, 2L, 1L, 3L)
        }
    })
