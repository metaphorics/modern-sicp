// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4.4.4, rules and unification: the book's unifier
// cases seen through the driver -- the chain that resolves three names to
// one, the frame that stores a term whose variables bind later, the case
// whose binding is already decided, and the failed unification that
// answers no frame -- plus the circular binding a unifier must refuse.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QAnd
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4RulesTest :
    FunSpec({
        val x = variable("x")
        val y = variable("y")
        val z = variable("z")

        test("the book's successful unifications, resolved through the frame") {
            val database = QueryDatabase()
            // (?x a ?y) against the stored (?y ?z a): ?x takes ?y, and the
            // chain resolves all three names to a
            database.assertFact(fact(terms(sym("t"), y, z, sym("a"))))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("t"), x, sym("a"), y)), listOf(x, y, z)) shouldBe
                listOf(
                    "?x = a",
                    "?y = a",
                    "?z = a",
                )
        }

        test("the frame stores a term whose variables bind later") {
            val database = QueryDatabase()
            // (?x ?x) against ((a ?y c) (a b ?z)): ?x stores (a ?y c), and
            // the second position binds ?y and ?z, the book's inference
            database.assertFact(fact(terms(sym("p"), terms(sym("a"), y, sym("c")), terms(sym("a"), sym("b"), z))))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("p"), x, x)), listOf(x, y, z)) shouldBe
                listOf(
                    "?x = [a, b, c]",
                    "?y = b",
                    "?z = c",
                )
        }

        test("a binding already decided answers the book's non-match") {
            val database = QueryDatabase()
            // (?x ?y a) against (?x b ?y): ?y would have to be both b and a
            database.assertFact(fact(terms(sym("w"), x, sym("b"), y)))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("w"), x, y, sym("a"))), listOf(x)) shouldBe emptyList()
        }

        test("a circular binding is refused rather than built") {
            val database = QueryDatabase()
            database.assertFact(fact(terms(sym("circ"), y, terms(sym("f"), y))))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("circ"), x, x)), listOf(x)) shouldBe emptyList()
        }

        test("renaming makes rule variables unique per application") {
            val database = QueryDatabase()
            database.addRule(rule(terms(sym("same"), x, x), pattern(terms(sym("seed"), x))))
            database.assertFact(fact(terms(sym("seed"), sym("a"))))
            database.assertFact(fact(terms(sym("seed"), sym("b"))))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("same"), x, x)), listOf(x)) shouldBe
                listOf(
                    "?x = a",
                    "?x = b",
                )
            answerLines(driver, pattern(terms(sym("same"), x, sym("b"))), listOf(x)) shouldBe listOf("?x = b")
        }
    })
