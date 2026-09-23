// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.9

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_09Test :
    FunSpec({
        test("Exercise 2.9: two width-1 intervals at different centers multiply to different widths").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(4.0 to 22.0, ex_2_09())
        }
    })
