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
    })
