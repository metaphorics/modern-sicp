// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.1.3

package sicp.ch2.examples

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * A pair needs no record: [makeProceduralPair] closes over its two values
 * and returns a function that dispatches on a selector, `0` for the first
 * component and `1` for the second. The two accessor functions send that
 * selector; either representation must recover both original values.
 */
public fun makeProceduralPair(
    first: Long,
    second: Long,
): (Long) -> Long {
    fun dispatch(selector: Long): Long =
        when (selector) {
            0L -> first
            1L -> second
            else -> error("Pair selector must be 0 or 1, got $selector")
        }
    return ::dispatch
}

public fun firstFromProceduralPair(pair: (Long) -> Long): Long = pair(0L)

public fun secondFromProceduralPair(pair: (Long) -> Long): Long = pair(1L)

public class S2_1_3ProceduralDataTest :
    FunSpec({
        test("the procedural representation recovers both captured values") {
            val pair = makeProceduralPair(1L, 2L)
            firstFromProceduralPair(pair) shouldBe 1L
            secondFromProceduralPair(pair) shouldBe 2L
        }
        test("the dispatcher rejects an unknown selector") {
            shouldThrow<IllegalStateException> { makeProceduralPair(1L, 2L)(2L) }
        }
    })
