// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.64

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_64Test :
    FunSpec({
        test("listToTree of [1, 3, 5, 7, 9, 11] builds the exact balanced shape partialTree specifies") {
            val expected =
                makeTree(
                    5L,
                    makeTree(1L, SetTree.Empty, makeTree(3L, SetTree.Empty, SetTree.Empty)),
                    makeTree(9L, makeTree(7L, SetTree.Empty, SetTree.Empty), makeTree(11L, SetTree.Empty, SetTree.Empty)),
                )
            listToTree(listOf(1L, 3L, 5L, 7L, 9L, 11L)) shouldBe expected
        }
        test("listToTree round-trips through treeToList1 back to the original ordered list") {
            val elements = listOf(1L, 3L, 5L, 7L, 9L, 11L)
            treeToList1(listToTree(elements)) shouldBe elements
        }
        test("listToTree of the empty list is the empty tree") {
            listToTree(emptyList()) shouldBe SetTree.Empty
        }
        test("listToTree of a single element is a single node") {
            listToTree(listOf(4L)) shouldBe makeTree(4L, SetTree.Empty, SetTree.Empty)
        }
        test("ex_2_64 round-trips to the original six elements") {
            treeToList1(ex_2_64()) shouldBe listOf(1L, 3L, 5L, 7L, 9L, 11L)
        }
    })
