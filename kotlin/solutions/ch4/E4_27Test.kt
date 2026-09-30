// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.27: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyModule

public class E427Test :
    FunSpec({
        test("Exercise 4.27: the delayed computation runs at first demand and never again") {
            lazyIdentityTranscript() shouldBe "1\n10\n2\n10\n2\n"
        }

        test("Exercise 4.27: the memoized re-force is an attempt without a computation") {
            val run = LazyModule.run(LAZY_IDENTITY_PROGRAM).fold({ e -> throw AssertionError(e.toString()) }, { it })
            run.result.error shouldBe null
            run.forcing.forceAttempts shouldBe 4L
            run.forcing.computations shouldBe 3L
        }
    })
