// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VInt

public class E3_20Test :
    FunSpec({
        test("Exercise 3.20: the mutation through alias is visible from x").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val x = proceduralCons(VInt(1L), VInt(2L))
            val alias = x
            alias.setCar(VInt(17L))
            org.junit.jupiter.api.Assertions
                .assertEquals(VInt(17L), x.car())
        }
    })
