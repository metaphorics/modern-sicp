// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.6, internal definitions: mutual recursion under
// the sequential define rule. The definitions come first and define only
// lambda values, so the names are in place by the time either body runs --
// the accidental-but-reliable case the section describes.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.OutputSink
import sicp.ch4.runProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.VBool

private const val MUTUAL = """
(define (f x)
  (define (even? n)
    (if (= n 0) true (odd? (- n 1))))
  (define (odd? n)
    (if (= n 0) false (even? (- n 1))))
  (even? x))
"""

public class S4_1_6InternalDefinitionsTest :
    FunSpec({
        test("mutually recursive internal definitions work as written") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    MUTUAL.trimIndent() +
                        """
                        (f 10)
                        (f 7)
                        """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "#t\n#f\n"
        }

        test("the names live in the call frame, so two calls do not interfere") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    MUTUAL.trimIndent() +
                        """
                        (f 6)
                        (f 6)
                        (f 5)
                        """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "#t\n#t\n#f\n"
        }

        test("an unassigned read is the typed fault exercise 4.16 installs") {
            // the base evaluator does not check *unassigned*; that is 4.16's edit
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    (define (g)
                      (define a b)
                      (define b 5)
                      a)
                    (g)
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "Error: unbound variable: b\n"
        }
    })
