// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.1.2

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * An alternate rational-number representation: [makeRatLazy] stores the pair
 * exactly as given, and the reduction by [gcd] happens on every access, in
 * [numerLazy] and [denomLazy], instead of once at construction. Nothing
 * above the pair-selector abstraction barrier, `addRat` and friends among
 * them, has to change to see this swap; that is the abstraction barrier's
 * whole point.
 */
public typealias RationalPair = Pair<Long, Long>

private tailrec fun gcd(
    a: Long,
    b: Long,
): Long = if (b == 0L) a else gcd(b, a % b)

public fun makeRatLazy(
    n: Long,
    d: Long,
): RationalPair = n to d

public fun numerLazy(x: RationalPair): Long = x.first / gcd(x.first, x.second)

public fun denomLazy(x: RationalPair): Long = x.second / gcd(x.first, x.second)

public class S2_1_2AbstractionBarriersTest :
    FunSpec({
        test("numerLazy and denomLazy reduce on every access, from an unreduced pair") {
            val x = makeRatLazy(6L, 9L)
            numerLazy(x) shouldBe 2L
            denomLazy(x) shouldBe 3L
        }
        test("the unreduced pair itself is unchanged; only the selectors reduce") {
            makeRatLazy(6L, 9L) shouldBe Pair(6L, 9L)
        }
    })
