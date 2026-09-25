// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.52

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_52Test :
    FunSpec({
        test("Exercise 3.52: the memoized script leaves sum at 1, 6, 10, 136, then 210") {
            val probe = AccumProbe()
            val seq = streamMap({ probe.accum(it) }, streamEnumerateInterval(1, 20))
            probe.sum shouldBe 1L
            probe.events shouldBe listOf("accum 1 -> 1")

            val y = streamFilter({ it % 2 == 0L }, seq)
            probe.sum shouldBe 6L

            val z = streamFilter({ it % 5 == 0L }, seq)
            probe.sum shouldBe 10L

            streamRef(y, 7) shouldBe 136L
            probe.sum shouldBe 136L

            displayStream(z) shouldBe "10\n15\n45\n55\n105\n120\n190\n210\n"
            probe.sum shouldBe 210L
        }

        test("Exercise 3.52: without the memoized tail every response changes") {
            val probe = AccumProbe()
            val run = coldAccumScript(probe)
            run.y shouldBe listOf(6L, 24L, 30L, 54L, 64L, 100L, 114L, 162L, 180L)
            run.z shouldBe listOf(15L, 230L, 245L, 275L, 300L, 345L, 380L)
            run.finalSum shouldBe 419L
            probe.sum shouldBe 419L
        }
    })
