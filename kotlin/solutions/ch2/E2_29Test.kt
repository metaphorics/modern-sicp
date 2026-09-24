// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.29

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_29Test :
    FunSpec({
        val balancedMobile =
            makeMobile(
                makeBranch(4L, Weight(4L)),
                makeBranch(2L, makeMobile(makeBranch(2L, Weight(6L)), makeBranch(6L, Weight(2L)))),
            )
        test("totalWeight of the sample mobile is 12") {
            ex_2_29() shouldBe 12L
        }
        test("the torque-matched mobile is balanced") {
            balanced(balancedMobile) shouldBe true
        }
        test("the sample mobile is not balanced") {
            balanced(sampleMobile()) shouldBe false
        }
        test("a lone weight is balanced") {
            balanced(Weight(7L)) shouldBe true
        }
        test("the pair representation gives the same total weight") {
            val consMobile =
                consMakeMobile(
                    consMakeBranch(2L, ConsStructure.ConsWeight(3L)),
                    consMakeBranch(
                        2L,
                        ConsStructure.ConsSubMobile(
                            consMakeMobile(
                                consMakeBranch(1L, ConsStructure.ConsWeight(4L)),
                                consMakeBranch(1L, ConsStructure.ConsWeight(5L)),
                            ),
                        ),
                    ),
                )
            consTotalWeight(ConsStructure.ConsSubMobile(consMobile)) shouldBe totalWeight(sampleMobile())
        }
    })
