// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4.4.5, maintaining the data base: the chronological
// collections behind the answer stream, the leading-symbol index that
// scopes a pattern to its own bucket, and the bind-then-append discipline
// of `add-assertion!` -- a new assertion lands last, never in front.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4DatabaseTest :
    FunSpec({
        val x = variable("x")
        val y = variable("y")
        val z = variable("z")

        test("the indexed bucket answers the stored assertions chronologically") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val names = answerLines(driver, pattern(terms(sym("job"), x, y)), listOf(x))
            names shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Hacker, Alyssa, P]",
                    "?x = [Fect, Cy, D]",
                    "?x = [Tweakit, Lem, E]",
                    "?x = [Reasoner, Louis]",
                    "?x = [Warbucks, Oliver]",
                    "?x = [Scrooge, Eben]",
                    "?x = [Cratchet, Robert]",
                    "?x = [Aull, DeWitt]",
                )
        }

        test("a new assertion is appended, not added in front") {
            val database = microshaftDatabase()
            database.assertFact(fact(terms(sym("job"), person("Doakes", "Donna"), terms(sym("computer"), sym("programmer")))))
            val driver = QueryDriver.streaming(database)
            val names = answerLines(driver, pattern(terms(sym("job"), x, terms(sym("computer"), sym("programmer")))), listOf(x))
            names.first() shouldBe "?x = [Hacker, Alyssa, P]"
            names.last() shouldBe "?x = [Doakes, Donna]"
        }

        test("an unindexed pattern fetches every assertion") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            driver
                .run(pattern(dotted(listOf(x), y)), listOf(x))
                .take(40)
                .toList()
                .size shouldBe 40
        }

        test("a variable-headed rule stays in scope for a constant-headed query") {
            val database = QueryDatabase()
            database.addRule(rule(terms(x, sym("derivative"), y), pattern(terms(sym("same"), x, y))))
            database.assertFact(fact(terms(sym("same"), sym("f"), sym("g"))))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("f"), sym("derivative"), z)), listOf(z)) shouldBe
                listOf("?z = g")
        }
    })
