// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_26

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_26Test :
    FunSpec({
        test("iterative depth is independent of n and pushes are linear") {
            // Section 5.4.4 with the tail-call controller: constant depth, linear pushes.
            val report = iterativeFactorialMeasurements()
            val depthRows = report.filter { it.startsWith("iterative factorial n=") }
            depthRows.map { it.substringBefore(":") } shouldBe
                (1..6).map { n -> "iterative factorial n=$n" }
            val depths =
                depthRows.map { row ->
                    Regex("""maximum-depth = (\d+)""")
                        .find(row)
                        ?.groupValues
                        ?.get(1)
                        ?.toInt()
                }
            depths.all { it != null && it >= 0 } shouldBe true
            depths.distinct().size shouldBe 1
            report.any { it == "maximum depth independent of n: true" } shouldBe true
            report.last() shouldBe "total pushes linear in n: true"
        }
    })
