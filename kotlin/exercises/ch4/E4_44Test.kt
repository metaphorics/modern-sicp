// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.44

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_44Test :
    FunSpec({
        test("Exercise 4.44: the first eight-queens solution").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            queensFirst(8) shouldBe "[4, 2, 7, 3, 6, 8, 5, 1]"
        }

        test("Exercise 4.44: the first four-queens solution").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            queensFirst(4) shouldBe "[3, 1, 4, 2]"
        }

        test("Exercise 4.44: the first six-queens solution").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            queensFirst(6) shouldBe "[5, 3, 1, 6, 4, 2]"
        }

        test("Exercise 4.44: backtracks to the first solution").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            queensBacktracks(4) shouldBe 22L
            queensBacktracks(5) shouldBe 10L
            queensBacktracks(6) shouldBe 165L
            queensBacktracks(8) shouldBe 868L
        }
    })
