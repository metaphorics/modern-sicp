// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_46

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_46Test :
    FunSpec({
        test("Exercise 5.46 reports compiled, evaluator, and special-purpose machine counters") {
            // The rows follow each engine's own calling convention; this test pins no cross-engine ordering.
            val report = fibonacciStackReport()

            fun counters(prefix: String): List<Long> {
                val line = report.single { it.startsWith(prefix) }
                val match =
                    Regex("""instructions = (\d+), total-pushes = (\d+), maximum-depth = (\d+)""")
                        .find(line) ?: error("missing machine counters: $line")
                return match.groupValues.drop(1).map { it.toLong() }
            }

            for (
            prefix in
            listOf("compiled fib(5):", "eceval fib(5):", "special-purpose fib(5):")
            ) {
                val measured = counters(prefix)
                (measured[0] > 0L) shouldBe true
                (measured[1] > 0L) shouldBe true
                (measured[2] > 0L && measured[2] <= measured[1]) shouldBe true
            }
        }
    })
