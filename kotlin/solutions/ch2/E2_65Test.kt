// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_65Test :
    FunSpec({
        val t1 = listToTree(listOf(1L, 3L, 5L, 7L, 9L))
        val t2 = listToTree(listOf(3L, 5L, 7L, 9L, 11L))

        test("unionSetTree contains every element of both trees, in order") {
            treeToList1(unionSetTree(t1, t2)) shouldBe listOf(1L, 3L, 5L, 7L, 9L, 11L)
        }
        test("intersectionSetTree contains exactly the shared elements") {
            treeToList1(intersectionSetTree(t1, t2)) shouldBe listOf(3L, 5L, 7L, 9L)
        }
        test("unionSetTree and intersectionSetTree with the empty tree") {
            treeToList1(unionSetTree(SetTree.Empty, t1)) shouldBe treeToList1(t1)
            treeToList1(intersectionSetTree(SetTree.Empty, t1)) shouldBe emptyList()
        }
        test("ex_2_65 matches the hand-computed union and intersection") {
            ex_2_65() shouldBe (listOf(1L, 3L, 5L, 7L, 9L, 11L) to listOf(3L, 5L, 7L, 9L))
        }
    })
