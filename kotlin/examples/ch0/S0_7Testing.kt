// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

/** The function under test: a table below, then a property. */
public fun parity(n: Long): String = if (n % 2L == 0L) "even" else "odd"

/** |a - b| without a negative intermediate: commutative by construction. */
public fun absDiff(
    a: Long,
    b: Long,
): Long = if (a > b) a - b else b - a

public class S0_7TestingTest :
    FunSpec({
        listOf(
            0L to "even",
            1L to "odd",
            42L to "even",
        ).forEach { (n, expected) ->
            test("parity($n) is $expected") {
                parity(n) shouldBe expected
            }
        }
        test("a property test runs the assertion over generated inputs") {
            checkAll(
                Arb.long(-100L, 100L),
                Arb.long(-100L, 100L),
            ) { a, b ->
                absDiff(a, b) shouldBe absDiff(b, a)
            }
        }
        test("boundary cases stay explicit") {
            absDiff(0L, 0L) shouldBe 0L
            absDiff(Long.MAX_VALUE, Long.MAX_VALUE) shouldBe 0L
        }
    })
