// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.6

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_06Test :
    FunSpec({
        test("Exercise 3.6: resetting replays the same sequence").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val r = makeResettableRand(7UL)
            val first = r.generate()
            r.reset(7UL)
            org.junit.jupiter.api.Assertions
                .assertEquals(first, r.generate())
        }
    })
