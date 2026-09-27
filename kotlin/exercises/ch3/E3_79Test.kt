// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.79

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import sicp.runtime.take
import kotlin.math.PI
import kotlin.math.abs
import kotlin.math.sin

public class E3_79Test :
    FunSpec({
        test("Exercise 3.79: pinned first values of the oscillator y'' = -y").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val oscillator = solve2ndGeneral({ _, yy -> -yy }, 0.01, 0.0, 1.0)
            oscillator.take(5) shouldBe listOf(0.0, 0.01, 0.02, 0.029999, 0.039996000000000004)
        }

        test("Exercise 3.79: the solution tracks sin within four time steps over 300 steps").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val ys = solve2ndGeneral({ _, yy -> -yy }, 0.01, 0.0, 1.0).take(301)
            for (k in 0..300) {
                abs(ys[k] - sin(0.01 * k)) shouldBeLessThan 4.0 * 0.01
            }
            abs(ys[300] - sin(3.0)) shouldBeLessThan 0.02
        }

        test("Exercise 3.79: with f(d, y) = a d + b y it agrees with solve2nd").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            solve2ndGeneral({ d, yy -> -0.2 * d + -1.0 * yy }, 0.01, 1.0, 0.0).take(50) shouldBe
                solve2nd(-0.2, -1.0, 0.01, 1.0, 0.0).take(50)
        }
    })
