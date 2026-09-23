// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class LStreamTest :
    FunSpec({
        test("the tail generator runs once across many forces") {
            var calls = 0
            val s: LStream<Int> =
                consStream(1) {
                    calls++
                    consStream(2) { LStream.Empty }
                }
            val t1 = s.streamTail()
            val t2 = s.streamTail()
            calls shouldBe 1
            (t1 === t2) shouldBe true
        }

        test("take walks heads of an infinite stream") {
            val ones: LStream<Int> =
                run {
                    var self: LStream.Cons<Int>? = null
                    val s = consStream(1) { self ?: LStream.Empty }
                    self = s as LStream.Cons<Int>
                    s
                }
            ones.take(5) shouldBe listOf(1, 1, 1, 1, 1)
        }

        test("two cursors share one memoized spine") {
            var builds = 0
            val s =
                consStream(0) {
                    builds++
                    consStream(1) { LStream.Empty }
                }
            s.asSequence().first() shouldBe 0
            s.asSequence().first() shouldBe 0
            s.streamTail().streamHead() shouldBe 1
            builds shouldBe 1
        }

        test("the empty stream stays empty") {
            val e: LStream<Int> = LStream.Empty
            e.streamHead() shouldBe null
            e.streamTail() shouldBe LStream.Empty
            e.take(3) shouldBe emptyList()
        }
    })
