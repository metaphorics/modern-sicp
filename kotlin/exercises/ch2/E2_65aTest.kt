// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65a (addition)

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_65aTest :
    FunSpec({
        test("Exercise 2.65a: the worked example agrees with java.util.TreeSet").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_65a() shouldBe true
        }
    })
