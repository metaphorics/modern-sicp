// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.78

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import sicp.runtime.take
import kotlin.math.abs

public class E3_78Test :
    FunSpec({
        test("Exercise 3.78: pinned solution values of the damped oscillator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val y = solve2nd(-0.2, -1.0, 0.01, 1.0, 0.0)
            streamRef(y, 100) shouldBe 0.5710853715751386
            streamRef(y, 200) shouldBe -0.2621361136816313
            streamRef(y, 300) shouldBe -0.7313719317259952
            streamRef(y, 400) shouldBe -0.5064238606998512
        }

        test("Exercise 3.78: successive oscillation extrema decay under the damping").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val ys = solve2nd(-0.2, -1.0, 0.01, 1.0, 0.0).take(945)
            val peaks =
                listOf(0 to 315, 315 to 630, 630 to 945).map { (lo, hi) ->
                    ys.subList(lo, hi).maxOf { abs(it) }
                }
            peaks shouldBe listOf(1.0, 0.7408645010200705, 0.5488697360170923)
            peaks[2] shouldBeLessThan peaks[1]
            peaks[1] shouldBeLessThan peaks[0]
        }
    })
