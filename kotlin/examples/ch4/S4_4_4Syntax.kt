// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4.4.7, query syntax procedures: the typed
// constructors take the query language's shapes as ordinary domain data --
// symbols, variables, proper and improper lists, and bodyless rules -- and
// the pinned rendering round-trips `?x`, `[a, b | rest]`, and the answer
// line form `?name = <rendered term>`.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QFrame
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer
import sicp.ch4.renderTerm

public class S4_4_4SyntaxTest :
    FunSpec({
        test("the constructors take query shapes as ordinary data") {
            renderTerm(sym("9am")) shouldBe "9am"
            renderTerm(terms(sym("meeting"), sym("computer"), terms(sym("Wednesday"), sym("3pm")))) shouldBe
                "[meeting, computer, [Wednesday, 3pm]]"
            renderTerm(terms()) shouldBe "[]"
            renderTerm(dotted(listOf(sym("computer")), variable("type"))) shouldBe "[computer | ?type]"
        }

        test("the pinned term rendering names every node shape") {
            renderTerm(variable("x")) shouldBe "?x"
            renderTerm(terms(sym("a"), sym("b"), sym("c"))) shouldBe "[a, b, c]"
            renderTerm(dotted(listOf(sym("a"), sym("b")), sym("rest"))) shouldBe "[a, b | rest]"
        }

        test("the answer line contracts a renamed or unbound variable") {
            val renamed = QVar("x-7")
            renderAnswer(QFrame(mapOf(renamed to sym("a"))), listOf(renamed)) shouldBe listOf("?x-7 = a")
            renderAnswer(QFrame(emptyMap()), listOf(renamed)) shouldBe listOf("?x-7 = ?x-7")
        }

        test("a bodyless rule accepts a consistent ground conclusion") {
            val database = QueryDatabase()
            database.addRule(rule(terms(sym("same"), variable("x"), variable("x")), sicp.ch4.QAnd(emptyList())))
            val driver = QueryDriver.streaming(database)
            driver.run(pattern(terms(sym("same"), sym("a"), sym("a"))), emptyList()).toList().size shouldBe 1
            driver.run(pattern(terms(sym("same"), sym("a"), sym("b"))), emptyList()).toList().size shouldBe 0
        }
    })
