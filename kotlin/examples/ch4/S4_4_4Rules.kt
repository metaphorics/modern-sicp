// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.4, rules and unification: the book's unifier
// cases, the renaming discipline, `depends-on?` seeing through frame
// bindings, and `apply-a-rule` over the prose rules.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Frame
import sicp.ch4.bindingInFrame
import sicp.ch4.contractQuestionMark
import sicp.ch4.dependsOn
import sicp.ch4.extend
import sicp.ch4.makeNewVariable
import sicp.ch4.patternMatch
import sicp.ch4.patternVar
import sicp.ch4.unifyMatch
import sicp.runtime.take

public class S4_4_4RulesTest :
    FunSpec({
        test("the book's successful unifications") {
            val f1 = unifyMatch(patQuery("(?x a ?y)"), patQuery("(?y ?z a)"), Frame.Empty)
            // ?x unifies with ?y, ?y and ?z with a: resolving the chain,
            // all three are a.
            bindingInFrame(patternVar("x"), f1!!).toString() shouldBe "(? y)"
            bindingInFrame(patternVar("y"), f1).toString() shouldBe "a"
            bindingInFrame(patternVar("z"), f1).toString() shouldBe "a"

            val f2 = unifyMatch(patQuery("(?x ?x)"), patQuery("((a ?y c) (a b ?z))"), Frame.Empty)
            // the frame stores ?x = (a ?y c); the chain binds ?y and ?z, so
            // instantiating resolves ?x to (a b c) -- the book's inference.
            bindingInFrame(patternVar("x"), f2!!).toString() shouldBe "(a (? y) c)"
            bindingInFrame(patternVar("y"), f2).toString() shouldBe "b"
            bindingInFrame(patternVar("z"), f2).toString() shouldBe "c"

            val f3 = unifyMatch(patQuery("(?x a)"), patQuery("((b ?y) ?z)"), Frame.Empty)
            bindingInFrame(patternVar("x"), f3!!).toString() shouldBe "(b (? y))"
            bindingInFrame(patternVar("z"), f3).toString() shouldBe "a"
        }

        test("the book's failed unification") {
            unifyMatch(patQuery("(?x ?y a)"), patQuery("(?x b ?y)"), Frame.Empty) shouldBe null
        }

        test("depends-on? sees through frame bindings") {
            val direct = extend(patternVar("x"), patternVar("y"), Frame.Empty)
            dependsOn(patternVar("y"), patternVar("x"), direct) shouldBe false
            val chained = extend(patternVar("y"), patternVar("x"), direct)
            dependsOn(patternVar("y"), patternVar("x"), chained) shouldBe true
        }

        test("renaming makes rule variables unique per application") {
            val system = microshaftSystem()
            val rule = patQuery("(rule (wheel ?person) (and (supervisor ?x ?person)))")
            val first = system.renameVariablesIn(rule)
            val second = system.renameVariablesIn(rule)
            first.toString() shouldBe "(rule (wheel (? 1 person)) (and (supervisor (? 1 x) (? 1 person))))"
            second.toString() shouldBe "(rule (wheel (? 2 person)) (and (supervisor (? 2 x) (? 2 person))))"
            contractQuestionMark(makeNewVariable(patternVar("x"), 7)) shouldBe "?x-7"
        }

        test("apply-a-rule answers through the body") {
            val system = microshaftSystem()
            val rule =
                patQuery(
                    "(rule (lives-near ?person-1 ?person-2)\n" +
                        "      (and (address ?person-1 (?town . ?rest-1))\n" +
                        "           (address ?person-2 (?town . ?rest-2))\n" +
                        "           (not (same ?person-1 ?person-2))))",
                )
            val answers = system.applyARule(rule, patQuery("(lives-near ?x (Bitdiddle Ben))"), Frame.Empty).take(9)
            answers.size shouldBe 2
        }

        test("a pattern with a stored variable value survives re-matching") {
            val frame = Frame.Empty.extended(patternVar("x"), readQuery("(f ?y)"))
            patternMatch(readQuery("(f ?y)"), readQuery("(f ?y)"), frame) shouldBe frame
        }
    })
