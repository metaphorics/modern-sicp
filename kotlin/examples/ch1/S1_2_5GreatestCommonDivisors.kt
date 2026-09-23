// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.2.5

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** Euclid's Algorithm: an iterative process, `tailrec`-verified by the compiler. */
public tailrec fun gcd(
    a: Long,
    b: Long,
): Long = if (b == 0L) a else gcd(b, a % b)

public class S1_2_5GreatestCommonDivisorsTest :
    FunSpec({
        test("gcd reduces the pair the text walks through step by step") {
            gcd(206L, 40L) shouldBe 2L
        }
        test("gcd agrees with the definition on a smaller pair") {
            gcd(16L, 28L) shouldBe 4L
        }
    })
