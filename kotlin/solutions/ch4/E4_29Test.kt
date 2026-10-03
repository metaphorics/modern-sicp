// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.29: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyModule

public class E429Test :
    FunSpec({
        test("Exercise 4.29: the memoized session counts 1 then 2") {
            memoizedCountsTranscript() shouldBe "100\n1\n1000\n2\n"
        }

        test("Exercise 4.29: a recomputation at every demand counts 2 then 5") {
            unmemoizedCountsTranscript() shouldBe "100\n2\n1000\n5\n"
        }

        test("Exercise 4.29: the instrument shows re-forces answered from the memo") {
            val run = LazyModule.run(MEMOIZED_COUNTS_PROGRAM).fold({ e -> throw AssertionError(e.toString()) }, { it })
            run.result.error shouldBe null
            (run.forcing.computations < run.forcing.forceAttempts) shouldBe true
        }
    })
