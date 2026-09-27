// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.2

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import kotlin.math.sqrt

public class E3_02Test :
    FunSpec({
        test("Exercise 3.2: a monitored sqrt counts its calls").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val s = makeMonitored<Double, Double> { x -> sqrt(x) }
            org.junit.jupiter.api.Assertions
                .assertEquals(10.0, s.call(100.0))
            org.junit.jupiter.api.Assertions
                .assertEquals(1, s.howManyCalls())
        }
    })
