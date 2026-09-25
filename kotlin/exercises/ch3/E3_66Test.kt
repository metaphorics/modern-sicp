// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.66

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.take
import java.math.BigInteger

/** The 0-based index the 3.66 laws predict for `(i, j)` with `i <= j`. */
private fun lawIndex(
    i: Long,
    j: Long,
): Long =
    when {
        i == j -> (1L shl i.toInt()) - 2
        i == 1L -> 2 * j - 3
        else -> (1L shl i.toInt()) * (j - i) + (1L shl (i.toInt() - 1)) - 2
    }

public class E3_66Test :
    FunSpec({
        test("Exercise 3.66: the walked 0-based indices match the pinned positions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            positionOf(1L to 2L) shouldBe 1L
            positionOf(2L to 2L) shouldBe 2L
            positionOf(3L to 3L) shouldBe 6L
            positionOf(2L to 6L) shouldBe 16L
            positionOf(1L to 100L) shouldBe 197L
        }

        test("Exercise 3.66: the first row (1, j) sits at 0-based index 2j - 3").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            for (j in 2L..40L) {
                positionOf(1L to j) shouldBe 2 * j - 3
            }
        }

        test("Exercise 3.66: the diagonal (k, k) sits at 0-based index 2^k - 2").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            for (k in 1..16) {
                positionOf(k.toLong() to k.toLong()) shouldBe (1L shl k) - 2
            }
        }

        test("Exercise 3.66: every pair in a 2000-element prefix obeys the index laws").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            pairs(integers, integers).take(2000).forEachIndexed { index, p ->
                lawIndex(p.first, p.second) shouldBe index.toLong()
            }
        }

        test("Exercise 3.66: (99, 100) sits at 2^99 + 2^98 - 2, past any walk").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val law =
                BigInteger.TWO
                    .pow(99)
                    .add(BigInteger.TWO.pow(98))
                    .subtract(BigInteger.TWO)
            law shouldBe BigInteger("950737950171172051122527404030")
            positionOf(99L to 100L) shouldBe null
        }

        test("Exercise 3.66: a pair with i > j never appears").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            positionOf(2L to 1L) shouldBe null
        }
    })
