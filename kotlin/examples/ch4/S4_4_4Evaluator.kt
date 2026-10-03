// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4.4.2, the evaluator: the data-directed dispatch
// over the query kinds -- the series `and`, the `not` filter, the typed
// guard standing in for the value filter, and the `always-true` handler that
// passes every frame of a bodyless rule.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QAnd
import sicp.ch4.QGuard
import sicp.ch4.QNot
import sicp.ch4.QSym
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4EvaluatorTest :
    FunSpec({
        val x = variable("x")

        test("and processes in series and not filters the frames") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val query =
                QAnd(
                    listOf(
                        pattern(terms(sym("supervisor"), x, person("Bitdiddle", "Ben"))),
                        QNot(pattern(terms(sym("job"), x, terms(sym("computer"), sym("technician"))))),
                    ),
                )
            answerLines(driver, query, listOf(x)) shouldBe
                listOf(
                    "?x = [Hacker, Alyssa, P]",
                    "?x = [Fect, Cy, D]",
                )
        }

        test("a guard filters over the underlying comparison") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val amount = variable("amount")
            val query =
                QAnd(
                    listOf(
                        pattern(terms(sym("salary"), x, amount)),
                        QGuard({ bound -> (bound.get(0) as QSym).name.toLong() > 75000L }, listOf(amount)),
                    ),
                )
            answerLines(driver, query, listOf(x)) shouldBe
                listOf(
                    "?x = [Warbucks, Oliver]",
                )
        }

        test("always-true passes the frames of a bodyless rule") {
            val database = QueryDatabase()
            database.addRule(rule(terms(sym("same"), variable("a"), variable("a")), QAnd(emptyList())))
            val driver = QueryDriver.streaming(database)
            val a = variable("a")
            val b = variable("b")
            driver.run(pattern(terms(sym("same"), a, a)), listOf(a)).toList().size shouldBe 1
            driver.run(pattern(terms(sym("same"), a, b)), listOf(a, b)).toList().size shouldBe 1
            // the conclusion's consistency still rejects the clash
            driver.run(pattern(terms(sym("same"), sym("Ben"), sym("Fect"))), emptyList()).toList().size shouldBe 0
        }
    })
