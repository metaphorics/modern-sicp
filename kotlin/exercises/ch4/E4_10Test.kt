// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10

package sicp.ch4.exercises

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.ch4.printValue
import sicp.ch4.readDatum

public class E4_10Test :
    FunSpec({
        test("Exercise 4.10: defun rewrites to the define sugar and fun to lambda").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            either { printValue(syntaxize(readDatum("(defun cube (x) (* x x x))"))) }.getOrNull() shouldBe
                "(define (cube x) (* x x x))"
            either { printValue(syntaxize(readDatum("(fun (x y) (+ x y))"))) }.getOrNull() shouldBe
                "(lambda (x y) (+ x y))"
        }

        test("Exercise 4.10: the transformed program runs, nested fun included").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            defunTranscript() shouldBe "343\n512\n"
        }

        test("Exercise 4.10: without the transform, defun is an unbound head").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            defunPlainTranscript() shouldBe "Error: unbound variable: defun\n"
        }
    })
