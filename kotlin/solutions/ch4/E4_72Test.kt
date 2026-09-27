// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.72

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_72Test :
    FunSpec({
        test("Exercise 4.72: interleave versus append") {
            interleaveVersusAppend() shouldBe
                listOf(
                    "interleave_first8",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves ?a ?b) (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))",
                    "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves ?a ?b) (supervisor (Fect Cy D) (Bitdiddle Ben)))",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves ?a ?b) (supervisor (Tweakit Lem E) (Bitdiddle Ben)))",
                    "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "interleave_supervisor_answers_in_first8=3",
                    "append_first8",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle Ben)))",
                    "append_supervisor_answers_in_first8=0",
                    "append_supervisor_answers_in_first20=0",
                )
        }
    })
