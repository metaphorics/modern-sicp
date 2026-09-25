// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.7, query syntax procedures: the shared D23
// reader takes the query language's shapes as ordinary data -- the empty
// list, dotted patterns, and tokens such as `9am` -- and the syntax
// transformations round-trip `?x`, `(? 7 x)`, and bodyless rules.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.contractQuestionMark
import sicp.ch4.isAssertionToBeAdded
import sicp.ch4.isRule
import sicp.ch4.makeNewVariable
import sicp.ch4.patternVar
import sicp.ch4.printValue
import sicp.ch4.querySyntaxProcess
import sicp.ch4.readDatum
import sicp.ch4.ruleBodyOf
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VSym

private fun read(text: String): sicp.runtime.Value =
    either {
        readDatum(text)
    }.fold({ e -> throw IllegalStateException(e.toString()) }, { it })

public class S4_4_4SyntaxTest :
    FunSpec({
        test("the shared reader takes query-language shapes as data") {
            read("()") shouldBe VNil
            read("(computer . ?type)").toString() shouldBe "(computer . ?type)"
            read("9am").toString() shouldBe "9am"
            read("(meeting computer (Wednesday 3pm))").toString() shouldBe "(meeting computer (Wednesday 3pm))"
            read(";; the comment\n(job ?x ?y)").toString() shouldBe "(job ?x ?y)"
            read("60000") shouldBe VInt(60000)
            read("(assert! (job (Bitdiddle Ben) (computer wizard)))").toString() shouldBe
                "(assert! (job (Bitdiddle Ben) (computer wizard)))"
        }

        test("query-syntax-process expands pattern variables") {
            printValue(querySyntaxProcess(read("(job ?x ?y)"))) shouldBe "(job (? x) (? y))"
            printValue(querySyntaxProcess(read("(job ?x (computer . ?type))"))) shouldBe
                "(job (? x) (computer . (? type)))"
            printValue(querySyntaxProcess(read("(salary (Bitdiddle Ben) 60000)"))) shouldBe
                "(salary (Bitdiddle Ben) 60000)"
        }

        test("make-new-variable and contract-question-mark round-trip") {
            val renamed = makeNewVariable(patternVar("x"), 7)
            renamed.toString() shouldBe "(? 7 x)"
            contractQuestionMark(renamed) shouldBe "?x-7"
            contractQuestionMark(patternVar("x")) shouldBe "?x"
            VSym("?").toString() shouldBe "?"
        }

        test("rule and assertion syntax") {
            isRule(read("(rule (same ?x ?x))")) shouldBe true
            isRule(read("(same ?x ?x)")) shouldBe false
            printValue(ruleBodyOf(read("(rule (same ?x ?x))"))) shouldBe "(always-true)"
            printValue(ruleBodyOf(read("(rule (wheel ?p) (and (supervisor ?m ?p)))")))
            "(and (supervisor ?m ?p))"
            isAssertionToBeAdded(read("(assert! (same ?x ?x))")) shouldBe true
            isAssertionToBeAdded(read("(same ?x ?x)")) shouldBe false
        }
    })
