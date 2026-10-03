// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_52

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_52Test :
    FunSpec({
        test("Exercise 5.52: the exercise's C backend emits a real C function over the compilation") {
            compiledCVerdicts(factorialProbeSource) shouldBe
                listOf(
                    "the emitter produced a C function: true",
                    "the artifact carries the compilation's labels and registers: true",
                    "the reference closure artifact answers like direct: true",
                )
        }
        test("Exercise 5.52: the exercise's emitted C program executes a recursive factorial") {
            compiledCRuns(factorialProbeSource) shouldBe
                listOf("120", "the emitted C answers like the direct run: true")
        }
        test("Exercise 5.52: the emitted C program reports the guest error category the engines report") {
            compiledCAgreement(errorProbeSource) shouldBe
                listOf(
                    "direct: 1, error DivisionByZero",
                    "explicit-control: 1, error DivisionByZero",
                    "compiled machine: 1, error DivisionByZero",
                    "emitted C: 1, error DivisionByZero",
                    "all four agree: true",
                )
        }
        test("Exercise 5.52: classes, methods, closures, maps, collections, and pairs run as emitted C") {
            val answers = "9.0 / moved to 9 / 31 / 103 / 2 / 56 / 2 / 1, no error"
            compiledCAgreement(structuredProbeSource) shouldBe
                listOf(
                    "direct: $answers",
                    "explicit-control: $answers",
                    "compiled machine: $answers",
                    "emitted C: $answers",
                    "all four agree: true",
                )
        }
        test("Exercise 5.52: the canonical self-interpreter of 5.50 runs as emitted C and agrees with every engine") {
            compiledCAgreement(metacircularEvaluatorSource) shouldBe
                listOf(
                    "direct: 120, no error",
                    "explicit-control: 120, no error",
                    "compiled machine: 120, no error",
                    "emitted C: 120, no error",
                    "all four agree: true",
                )
        }
    })
