// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.5

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The eager shape the translation gives Ben's test: `y` is a plain `Long`. */
public fun testEager(
    x: Long,
    y: Long,
): Long = if (x == 0L) 0L else y

/** The deferred shape: the operand arrives as a lambda, run only if needed. */
public fun testDeferred(
    x: Long,
    y: () -> Long,
): Long = if (x == 0L) 0L else y()

public class S1_1_5SubstitutionTest :
    FunSpec({
        test("the substitution model reduces f(5) to 136") {
            f(5L) shouldBe 136L
        }
        test("an eager operand runs although the taken branch ignores it") {
            var evaluations = 0

            fun operand(): Long {
                evaluations += 1
                return 7L
            }
            testEager(0L, operand()) shouldBe 0L
            evaluations shouldBe 1
        }
        test("a lambda operand runs only when the body invokes it") {
            var evaluations = 0
            testDeferred(0L) {
                evaluations += 1
                7L
            } shouldBe 0L
            evaluations shouldBe 0
        }
    })
