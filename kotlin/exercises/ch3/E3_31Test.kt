// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.31

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_31Test :
    FunSpec({
        test("Exercise 3.31: a probe prints one line at registration, before any signal changes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(2, registrationProbeLog().size)
        }
    })
