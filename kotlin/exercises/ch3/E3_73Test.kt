// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.73

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** A constant unit current, the book's steady charging signal. */
private val unitCurrents: LStream<Double> = consStream(1.0) { unitCurrents }

public class E3_73Test :
    FunSpec({
        test("Exercise 3.73: a constant current charges the capacitor in linear steps").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val rc1 = rc(5.0, 1.0, 0.5)
            rc1(unitCurrents, 0.0).take(8) shouldBe
                listOf(5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5)
        }

        test("Exercise 3.73: the initial capacitor voltage offsets the whole response").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val rc1 = rc(5.0, 1.0, 0.5)
            val current: LStream<Double> =
                consStream(1.0) { consStream(2.0) { consStream(3.0) { LStream.Empty } } }
            rc1(current, 2.0).take(3) shouldBe listOf(7.0, 12.5, 18.5)
        }
    })
