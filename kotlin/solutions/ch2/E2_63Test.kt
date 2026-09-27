// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.63

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_63Test :
    FunSpec({
        val expected = listOf(1L, 3L, 5L, 7L, 9L, 11L)

        test("treeToList1 and treeToList2 agree, and produce the sorted set, for every Figure 2.16 shape") {
            ex_2_63().forEach { it shouldBe expected }
        }
        test("treeToList1 of the empty tree is the empty list") {
            treeToList1(SetTree.Empty) shouldBe emptyList()
        }
        test("treeToList2 of the empty tree is the empty list") {
            treeToList2(SetTree.Empty) shouldBe emptyList()
        }
        test("treeToList1 and treeToList2 agree on a single-node tree") {
            val single = makeTree(4L, SetTree.Empty, SetTree.Empty)
            treeToList1(single) shouldBe listOf(4L)
            treeToList2(single) shouldBe listOf(4L)
        }
    })
