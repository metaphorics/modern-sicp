// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.74

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_74Test :
    FunSpec({
        test("ex_2_74 finds both employees' salaries across two differently shaped division files") {
            ex_2_74() shouldBe Triple("60000", "80000", false)
        }

        test("getRecord tags a hit with the division it came from") {
            val file = DivisionFile.North(mapOf("Ben Bitdiddle" to mapOf("salary" to "60000")))
            getRecord("Ben Bitdiddle", file) shouldBe TaggedRecord("north", mapOf("salary" to "60000"))
        }

        test("getRecord answers null, not a false-ish sentinel, when a division has no such name") {
            val file = DivisionFile.South(listOf(listOf("name", "Alyssa P. Hacker", "salary", "80000")))
            getRecord("Ben Bitdiddle", file) shouldBe null
        }

        test("findEmployeeRecord searches every division's file in order until one hits") {
            val north = DivisionFile.North(mapOf("Ben Bitdiddle" to mapOf("salary" to "60000")))
            val south = DivisionFile.South(listOf(listOf("name", "Alyssa P. Hacker", "salary", "80000")))
            findEmployeeRecord("Alyssa P. Hacker", listOf(north, south)) shouldBe
                TaggedRecord("south", mapOf("name" to "Alyssa P. Hacker", "salary" to "80000"))
        }
    })
