// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.64

package sicp.ch3.exercises

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take
import kotlin.math.abs

/** A finite double stream. */
private fun doublesOf(vararg values: Double): LStream<Double> {
    fun build(i: Int): LStream<Double> = if (i == values.size) LStream.Empty else consStream(values[i]) { build(i + 1) }
    return build(0)
}

public class E3_64Test :
    FunSpec({
        test("sqrt by limit to 1e-8 lands on the machine square root") {
            val limit = sqrtByLimit(2.0, 1e-8)
            limit shouldBe 1.414213562373095
            abs(limit - Math.sqrt(2.0)).shouldBeLessThan(1e-9)
        }

        test("looser tolerances stop earlier at distinct convergences") {
            sqrtByLimit(2.0, 0.1) shouldBe 1.4166666666666665
            sqrtByLimit(2.0, 0.001) shouldBe 1.4142135623746899
        }

        test("a stream that runs dry before converging is an error") {
            shouldThrow<IllegalStateException> { streamLimit(LStream.Empty, 0.1) }
            shouldThrow<IllegalStateException> { streamLimit(doublesOf(1.0, 2.0, 3.0), 0.5) }
        }
    })
