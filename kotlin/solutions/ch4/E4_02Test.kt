// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_02Test :
    FunSpec({
        test("Exercise 4.2a: applications-first fails define in operator position, spares kernel forms") {
            louisDefineTranscript() shouldBe "error\n3\ntrue\n"
        }

        test("Exercise 4.2b: call-prefixed applications run") {
            callSugarTranscript() shouldBe "49\n"
        }
    })
