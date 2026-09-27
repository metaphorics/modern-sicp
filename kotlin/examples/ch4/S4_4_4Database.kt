// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.5, maintaining the data base: the chronological
// collections behind the stream interface, the leading-symbol index, the
// variable-conclusion bucket, and the bind-then-append discipline of
// `add-assertion!`.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QuerySystem
import sicp.ch4.indexKeyOf
import sicp.ch4.isIndexable
import sicp.ch4.printValue
import sicp.ch4.querySyntaxProcess
import sicp.ch4.useIndex
import sicp.runtime.VSym
import sicp.runtime.take

public class S4_4_4DatabaseTest :
    FunSpec({
        val system = microshaftSystem()

        test("fetch-assertions lists the indexed bucket chronologically") {
            val pattern = patQuery("(job ?x (computer programmer))")
            useIndex(pattern) shouldBe true
            indexKeyOf(pattern).toString() shouldBe "job"
            system.fetchAssertions(pattern).take(9).map { printValue(it) } shouldBe
                listOf(
                    "(job (Bitdiddle Ben) (computer wizard))",
                    "(job (Hacker Alyssa P) (computer programmer))",
                    "(job (Fect Cy D) (computer programmer))",
                    "(job (Tweakit Lem E) (computer technician))",
                    "(job (Reasoner Louis) (computer programmer trainee))",
                    "(job (Warbucks Oliver) (administration big wheel))",
                    "(job (Scrooge Eben) (accounting chief accountant))",
                    "(job (Cratchet Robert) (accounting scrivener))",
                    "(job (Aull DeWitt) (administration secretary))",
                )
        }

        test("a new assertion is appended, not consed in front") {
            val system = microshaftSystem()
            system.run("(assert! (job (Doakes Donna) (computer programmer)))")
            val answers = answersOf(system, "(job ?x (computer programmer))")
            answers.first() shouldBe "(job (Hacker Alyssa P) (computer programmer))"
            answers.last() shouldBe "(job (Doakes Donna) (computer programmer))"
        }

        test("an unindexed pattern fetches everything") {
            val pattern = patQuery("(?x next-to ?y)")
            useIndex(pattern) shouldBe false
            system.fetchAssertions(pattern).take(100).size shouldBe 39
        }

        test("a variable-headed conclusion is stored under the ? bucket") {
            val system = QuerySystem()
            val rule = patQuery("(rule (?x derivative ?y) (same ?x ?y))")
            val conclusion = patQuery("(?x derivative ?y)")
            isIndexable(conclusion) shouldBe true
            indexKeyOf(conclusion).toString() shouldBe "?"
            system.addRule(rule)
            // a constant-headed pattern still sees the variable-headed rule
            val pattern = patQuery("(f derivative ?z)")
            system.fetchRules(pattern).take(9).size shouldBe 1
            VSym("?").toString() shouldBe "?"
        }
    })
