// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** An expression body: the function is the expression after `=`. */
public fun square(x: Long): Long = x * x

/** A block body; the last expression, here the `return`, is the answer. */
public fun hypotenuseSquared(
    a: Long,
    b: Long,
): Long {
    val a2 = square(a)
    val b2 = square(b)
    return a2 + b2
}

/** A lambda closes over the `n` of its enclosing call. */
public fun makeAdder(n: Long): (Long) -> Long = { x -> x + n }

/** A type alias names a function shape. */
public typealias Mapper = (Long) -> Long

/** Procedures as arguments: `sumOver(square, 1, 3)` is 1 + 4 + 9. */
public fun sumOver(
    f: Mapper,
    a: Long,
    b: Long,
): Long = (a..b).fold(0L) { acc, k -> acc + f(k) }

/** Procedures as results: [twice] builds a new function from [m]. */
public fun twice(m: Mapper): Mapper = { x -> m(m(x)) }

public class S0_2FunctionsTest :
    FunSpec({
        test("expression and block bodies") {
            square(7) shouldBe 49L
            hypotenuseSquared(3, 4) shouldBe 25L
        }
        test("a lambda remembers the n it was made with") {
            val add5 = makeAdder(5L)
            val add12 = makeAdder(12L)
            add5(1L) shouldBe 6L
            add12(1L) shouldBe 13L
        }
        test("procedures as arguments and results") {
            sumOver(::square, 1L, 3L) shouldBe 14L
            val quad = twice(::square)
            quad(2L) shouldBe 16L
        }
    })
