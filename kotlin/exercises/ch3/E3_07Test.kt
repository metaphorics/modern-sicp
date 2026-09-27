// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.7

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_07Test :
    FunSpec({
        test("Exercise 3.7: the joint account's new password reaches the original account").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val peterAcc = makeAccount(100L, "open-sesame")
            val paulAcc = makeJoint(peterAcc, "open-sesame", "rosebud")
            org.junit.jupiter.api.Assertions
                .assertEquals(60L, paulAcc.withdraw("rosebud", 40L).getOrNull())
        }
    })
