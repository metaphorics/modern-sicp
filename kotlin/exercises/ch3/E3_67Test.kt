// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.67

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.collections.shouldContain
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_67Test :
    FunSpec({
        test("Exercise 3.67: both orders of a pair appear inside the first 20").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val prefix = pairsAll(integers, integers).take(20)
            prefix shouldContain (2L to 3L)
            prefix shouldContain (3L to 2L)
        }

        test("Exercise 3.67: the diagonal (i, i) appears, at 0-based index (4^i - 4) / 3").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val prefix = pairsAll(integers, integers).take(1500)
            for (i in 1L..6L) {
                prefix.indexOf(i to i) shouldBe (((1L shl (2 * i.toInt())) - 4) / 3).toInt()
            }
        }

        test("Exercise 3.67: the same prefix covers far more small-sum pairs than pairs").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            pairsAll(integers, integers).take(60).count { it.first + it.second <= 8L } shouldBe 24
            pairs(integers, integers).take(60).count { it.first + it.second <= 8L } shouldBe 16
        }
    })
