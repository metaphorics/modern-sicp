// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.63

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_63Test :
    FunSpec({
        test("walking Alyssa's stream costs one improve call per element") {
            val probe5 = SqrtProbe()
            streamRef(sqrtStreamShared(2.0, probe5), 5)
            probe5.improveCalls shouldBe 5

            val probe6 = SqrtProbe()
            streamRef(sqrtStreamShared(2.0, probe6), 6)
            probe6.improveCalls shouldBe 6
        }

        test("walking Louis's stream pays the triangular cascade") {
            val probe5 = SqrtProbe()
            streamRef(sqrtStreamFresh(2.0, probe5), 5)
            probe5.improveCalls shouldBe 15

            val probe6 = SqrtProbe()
            streamRef(sqrtStreamFresh(2.0, probe6), 6)
            probe6.improveCalls shouldBe 21
        }

        test("both variants produce the same guesses as the section's sqrt-stream") {
            val expected = sqrtStream(2.0).take(6)
            sqrtStreamShared(2.0, SqrtProbe()).take(6) shouldBe expected
            sqrtStreamFresh(2.0, SqrtProbe()).take(6) shouldBe expected
        }
    })
