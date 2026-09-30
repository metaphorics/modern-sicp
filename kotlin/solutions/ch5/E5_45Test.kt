// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_45

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_45Test :
    FunSpec({
        test("Exercise 5.45 reports compiled, evaluator, and special-purpose machine counters") {
            // The rows follow each engine's own calling convention; this test pins no cross-engine ordering.
            val report = factorialStackReport()

            fun counters(prefix: String): List<Long> {
                val line = report.single { it.startsWith(prefix) }
                val match =
                    Regex("""instructions = (\d+), total-pushes = (\d+), maximum-depth = (\d+)""")
                        .find(line) ?: error("missing machine counters: $line")
                return match.groupValues.drop(1).map { it.toLong() }
            }

            val compiled = counters("compiled factorial(5):")
            val evaluator = counters("eceval factorial(5):")
            val special = counters("special-purpose factorial(5):")
            for (measured in listOf(compiled, evaluator, special)) {
                (measured[0] > 0L) shouldBe true
                (measured[1] > 0L) shouldBe true
                (measured[2] > 0L && measured[2] <= measured[1]) shouldBe true
            }
            // Figure 5.11 has 2(n - 1) pushes and reaches that depth (§5.2, Exercise 5.14).
            special[1] shouldBe 8L
            special[2] shouldBe 8L
        }
    })
