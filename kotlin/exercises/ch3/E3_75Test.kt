// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.75

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/**
 * A noisy sensor sample: a slow drift through zero plus alternating
 * noise of half a unit. The drift alone crosses zero exactly once over
 * the 41 samples, which is what a correct smoother should report.
 */
private val noisySensor: LStream<Double> =
    (0..40)
        .map { k -> 0.4 - 0.02 * k + 0.5 * (if (k % 2 == 0) 1.0 else -1.0) }
        .foldRight(LStream.Empty as LStream<Double>) { x, acc -> consStream(x) { acc } }

public class E3_75Test :
    FunSpec({
        test("Exercise 3.75: Louis's one-thread version reports spurious crossings").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            makeZeroCrossingsBuggy(noisySensor, 0.0).take(41) shouldBe
                listOf(
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    1L,
                    -1L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                )
        }

        test("Exercise 3.75: the two-thread fix keeps only the true crossing").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            makeZeroCrossingsSmoothed(noisySensor, 0.0).take(41) shouldBe
                listOf(
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    -1L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                    0L,
                )
        }
    })
