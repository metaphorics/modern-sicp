// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.5.4, streams and delayed evaluation

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/** The book's delayed-integrand `integral` of 3.5.4: [delayedIntegrand]
 * is forced only past the first element, which is what lets `solve`'s
 * feedback loop close. The internal `int` stream is defined in terms of
 * itself; Kotlin locals cannot appear in their own initializer, so the
 * tail thunk reads the cell that the last line fills before anything can
 * force a tail. */
public fun integral(
    delayedIntegrand: Lazy<LStream<Double>>,
    initialValue: Double,
    dt: Double,
): LStream<Double> {
    var int: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(initialValue) {
            addStreams(scaleStream(delayedIntegrand.value, dt), checkNotNull(int) { "int not yet tied" })
        }
    int = tied
    return tied
}

/** The book's `solve` of 3.5.4: the equation dy/dt = f(y) as a feedback
 * loop. The delayed integrand is the book's `(delay dy)`, tied back to
 * `y` itself inside the lazy value. */
public fun solve(
    f: (Double) -> Double,
    y0: Double,
    dt: Double,
): LStream<Double> {
    var dy: LStream<Double>? = null
    val y: LStream<Double> = integral(lazy { checkNotNull(dy) { "dy not yet tied" } }, y0, dt)
    dy = streamMap(f, y)
    return y
}

public class S3_5_4DelayedEvaluationTest :
    FunSpec({
        test("solve approximates e: y(1) of dy/dt = y") {
            streamRef(solve({ y -> y }, 1.0, 0.001), 1000) shouldBe 2.716923932235896
        }

        test("the delayed integrand is forced only past the first element") {
            var forcings = 0
            val integrand =
                lazy {
                    forcings += 1
                    streamMap({ it.toDouble() }, ones)
                }
            val s = integral(integrand, 5.0, 1.0)
            forcings shouldBe 0
            streamRef(s, 0) shouldBe 5.0
            forcings shouldBe 0
            streamRef(s, 1) shouldBe 6.0
            forcings shouldBe 1
            streamRef(s, 2) shouldBe 7.0
            forcings shouldBe 1
        }

        test("solve itself starts with dy's head already mapped") {
            var fCalls = 0
            val y =
                solve({ y0 ->
                    fCalls += 1
                    y0
                }, 1.0, 1.0)
            fCalls shouldBe 1
            y.streamHead() shouldBe 1.0
            streamRef(y, 1) shouldBe 2.0
        }
    })
