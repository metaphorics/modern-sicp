// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.1.3

package sicp.ch2.examples

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * A pair needs no data structure at all: [cons] closes over its two
 * arguments and returns a function that dispatches on a message, `0` for
 * `car` and `1` for `cdr`. [car] and [cdr] just send that message. Any
 * representation of pairs has to satisfy one condition: for any `x` and
 * `y`, `car(cons(x, y))` is `x` and `cdr(cons(x, y))` is `y`.
 */
public fun cons(
    x: Long,
    y: Long,
): (Long) -> Long {
    fun dispatch(m: Long): Long =
        when (m) {
            0L -> x
            1L -> y
            else -> error("Argument not 0 or 1: CONS $m")
        }
    return ::dispatch
}

public fun car(z: (Long) -> Long): Long = z(0L)

public fun cdr(z: (Long) -> Long): Long = z(1L)

public class S2_1_3ProceduralDataTest :
    FunSpec({
        test("car and cdr recover exactly what cons closed over") {
            val z = cons(1L, 2L)
            car(z) shouldBe 1L
            cdr(z) shouldBe 2L
        }
        test("a dispatch function rejects any message other than 0 or 1") {
            shouldThrow<IllegalStateException> { cons(1L, 2L)(2L) }
        }
    })
