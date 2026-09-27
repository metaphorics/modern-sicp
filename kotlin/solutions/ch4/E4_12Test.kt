// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.solutions

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.parseExpr
import sicp.ch4.printValue
import sicp.ch4.readDatum
import sicp.runtime.Env

public class E4_12Test :
    FunSpec({
        test("Exercise 4.12: define, lookup, and set! run through the scan abstractions") {
            scannedTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.12: a binding that died with its call frame is unbound at top level") {
            scannedFreshFrameTranscript() shouldBe "2\nError: unbound variable: z\n"
        }

        test("Exercise 4.12: frameScan reads one frame, envScan walks the chain") {
            val sink = sicp.ch4.OutputSink()
            val evaluator = ScannedFrames(sink)
            either {
                evaluator.eval(parseExpr(readDatum("(define z 2)")), evaluator.global)
            }.getOrNull()
            frameScan("z", evaluator.global)?.let { printValue(it) } shouldBe "(z . 2)"
            frameScan("w", evaluator.global) shouldBe null
            envScan("z", Env.child(evaluator.global))?.let { printValue(it) } shouldBe "(z . 2)"
            envScan("w", evaluator.global) shouldBe null
        }

        test("Exercise 4.12: extend-environment keeps the arity contract on the scans") {
            scannedArityTranscript() shouldBe "Error: extend: wrong number of arguments, expected 2, got 1\n"
        }
    })
