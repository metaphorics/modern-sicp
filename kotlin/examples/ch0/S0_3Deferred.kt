// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** The probe counter of this listing: how many times the delayed work ran. */
public var lazyCalls: Int = 0

/** A `lazy` value computes at most once, at the first read. */
public val expensiveAnswer: Long by lazy {
    lazyCalls += 1
    40L + 2L
}

/** A cold pipeline: nothing runs until a terminal operation asks. */
public fun naturals(): Sequence<Long> = generateSequence(1L) { it + 1L }

/** Squares as a `Sequence`; each pass over it recomputes from the start. */
public fun squaresSeq(): Sequence<Long> = naturals().map { x -> x * x }

/** The `sequence { }` builder writes the generator as a suspended loop. */
public fun evens(): Sequence<Long> =
    sequence {
        var n = 0L
        while (true) {
            yield(n)
            n += 2L
        }
    }

/** The runtime's memoized stream: each tail computes at most once. */
public fun squareStream(n: Long): LStream<Long> = consStream(n * n) { squareStream(n + 1L) }

public class S0_3DeferredTest :
    FunSpec({
        test("lazy computes once across many reads") {
            lazyCalls = 0
            expensiveAnswer shouldBe 42L
            expensiveAnswer shouldBe 42L
            lazyCalls shouldBe 1
        }
        test("sequences compose cold and infinite pipelines") {
            squaresSeq().take(5).toList() shouldBe listOf(1L, 4L, 9L, 16L, 25L)
            evens().take(3).toList() shouldBe listOf(0L, 2L, 4L)
        }
        test("the memoized stream forces each tail once") {
            squareStream(1L).take(5) shouldBe listOf(1L, 4L, 9L, 16L, 25L)
        }
    })
