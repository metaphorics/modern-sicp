// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4.4.3, finding assertions by pattern matching: the
// book's matcher cases over `[[a, b], c, [a, b]]` -- the repeated variable
// matching consistently, the nested case, the non-match answering no
// frame, and a stored term containing a variable matched recursively.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4MatcherTest :
    FunSpec({
        val x = variable("x")
        val y = variable("y")
        val z = variable("z")
        val data = terms(terms(sym("a"), sym("b")), sym("c"), terms(sym("a"), sym("b")))

        test("the book's matches of [[a, b], c, [a, b]]") {
            val database = QueryDatabase()
            database.assertFact(fact(data))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(x, sym("c"), x)), listOf(x)) shouldBe listOf("?x = [a, b]")
            answerLines(driver, pattern(terms(x, y, z)), listOf(x, y, z)) shouldBe
                listOf(
                    "?x = [a, b]",
                    "?y = c",
                    "?z = [a, b]",
                )
            val nested = pattern(terms(terms(x, y), sym("c"), terms(x, y)))
            answerLines(driver, nested, listOf(x, y)) shouldBe
                listOf(
                    "?x = a",
                    "?y = b",
                )
        }

        test("the book's non-match") {
            val database = QueryDatabase()
            database.assertFact(fact(data))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(x, sym("a"), y)), listOf(x)) shouldBe emptyList()
        }

        test("a stored term containing a variable is matched recursively") {
            // the query (same (f ?y) (f b)) meets the stored (same ?x ?x):
            // ?x takes the term (f ?y), whose own match binds ?y to b
            val database = QueryDatabase()
            database.addRule(rule(terms(sym("same"), variable("x"), variable("x")), sicp.ch4.QAnd(emptyList())))
            val driver = QueryDriver.streaming(database)
            val query = pattern(terms(sym("same"), terms(sym("f"), y), terms(sym("f"), sym("b"))))
            answerLines(driver, query, listOf(y)) shouldBe listOf("?y = b")
        }
    })
