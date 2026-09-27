// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.1, the driver loop and instantiation: the
// prompts, the echoed query, the `assert!` command path, and the
// unbound-variable handler that contracts renamed pattern variables.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QuerySystem

public class S4_4_4DriverTest :
    FunSpec({
        test("the driver answers with instantiated queries") {
            val system = microshaftSystem()
            system.repl("(job ?x (computer technician))") shouldBe
                """
                ;;; Query input:
                (job ?x (computer technician))
                ;;; Query results:
                (job (Tweakit Lem E) (computer technician))
                """.trimIndent() + "\n"
        }

        test("an assert! is filed instead of answered") {
            val system = microshaftSystem()
            system.repl("(assert! (can-do-job (computer wizard) (computer programmer trainee)))") shouldBe
                """
                ;;; Query input:
                (assert! (can-do-job (computer wizard) (computer programmer trainee)))
                Assertion added to data base.
                """.trimIndent() + "\n"
            system.repl("(can-do-job (computer wizard) ?job)") shouldBe
                """
                ;;; Query input:
                (can-do-job (computer wizard) ?job)
                ;;; Query results:
                (can-do-job (computer wizard) (computer programmer))
                (can-do-job (computer wizard) (computer technician))
                (can-do-job (computer wizard) (computer programmer trainee))
                """.trimIndent() + "\n"
        }

        test("the unbound-variable handler contracts renamed variables") {
            val system = QuerySystem()
            system.load(proseRules)
            system.repl("(append-to-form ?x ?y (a b c d))") shouldBe
                """
                ;;; Query input:
                (append-to-form ?x ?y (a b c d))
                ;;; Query results:
                (append-to-form () (a b c d) (a b c d))
                (append-to-form (a) (b c d) (a b c d))
                (append-to-form (a b) (c d) (a b c d))
                (append-to-form (a b c) (d) (a b c d))
                (append-to-form (a b c d) () (a b c d))
                """.trimIndent() + "\n"
        }
    })
