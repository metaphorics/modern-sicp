// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.6

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Restore
import sicp.runtime.Save

public class E5_06Test :
    FunSpec({
        test("Exercise 5.6: the modified machine answers alike and sheds one save per level") {
            modifiedFibReport() shouldBe
                listOf(
                    "55",
                    "55",
                    "steps=281 saves=48",
                    "steps=257 saves=36",
                )
        }
        test("Exercise 5.6: the modification removes exactly the redundant pair") {
            val restores =
                fibController.withIndex().filter { (_, instruction) ->
                    instruction is Restore && instruction.reg == "continue"
                }
            val saves =
                fibController.withIndex().filter { (_, instruction) ->
                    instruction is Save && instruction.reg == "continue"
                }
            val redundantRestore = restores.first().index
            val redundantSave = saves[1].index
            fibController.filterIndexed { index, _ -> index != redundantRestore && index != redundantSave } shouldBe
                fibModifiedController
        }
        test("Exercise 5.6: both machines agree on n = 0 through 9") {
            val answers = (0L..9L).map { n -> fibAnswer(fibController, n) }
            val modifiedAnswers = (0L..9L).map { n -> fibAnswer(fibModifiedController, n) }
            answers shouldBe listOf("0", "1", "1", "2", "3", "5", "8", "13", "21", "34")
            modifiedAnswers shouldBe answers
        }
        test("Exercise 5.6: the tempting wrong pairing breaks the machine") {
            val wrongRestore = fibController.indexOfLast { it is Restore && it.reg == "continue" }
            val wrongSave =
                fibController
                    .withIndex()
                    .filter { (_, instruction) ->
                        instruction is Save && instruction.reg == "continue"
                    }[1]
                    .index
            fibController.filterIndexed { index, _ -> index != wrongRestore && index != wrongSave } shouldBe
                fibWrongPairController
            fibWrongPairEnding(2) shouldBe "stuck: UnassignedRegister at restore val"
        }
    })
