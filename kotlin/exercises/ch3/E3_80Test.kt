// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.80

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import sicp.runtime.take
import kotlin.math.abs

public class E3_80Test :
    FunSpec({
        test("Exercise 3.80: pinned capacitor voltage of the book's circuit").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val streams = rlc(1.0, 1.0, 0.2, 0.1)(10.0, 0.0)
            listOf(5, 10, 20, 30, 50).map { streamRef(streams.first, it) } shouldBe
                listOf(5.5955, -3.5160605800000004, -2.547583716974599, 4.59751360693207, 0.15526261365409222)
        }

        test("Exercise 3.80: pinned inductor current of the book's circuit").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val streams = rlc(1.0, 1.0, 0.2, 0.1)(10.0, 0.0)
            listOf(5, 10, 20, 30, 50).map { streamRef(streams.second, it) } shouldBe
                listOf(3.6461, 2.750945989, -2.691268933365921, 0.9857934876496868, -1.2231501590473282)
        }

        test("Exercise 3.80: successive voltage peaks decay as the resistor dissipates").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val vc = rlc(1.0, 1.0, 0.2, 0.1)(10.0, 0.0).first.take(90)
            val peaks =
                listOf(0 to 30, 30 to 60, 60 to 90).map { (lo, hi) ->
                    vc.subList(lo, hi).maxOf { abs(it) }
                }
            peaks shouldBe listOf(10.0, 4.59751360693207, 1.6854674625828383)
            peaks[2] shouldBeLessThan peaks[1]
            peaks[1] shouldBeLessThan peaks[0]
        }
    })
