// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.44

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_44Test :
    FunSpec({
        test("Exercise 2.44: upSplit(wave, 1) paints 3 copies of wave, 42 segments").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_44() shouldBe 42
        }
    })
