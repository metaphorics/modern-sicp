// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_78

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QuerySystem
import sicp.ch4.ambDriver

public class E4_78Test :
    FunSpec({
        test("Exercise 4.78: the amb port answers one at a time, depth-first") {
            ambQueryDemos() shouldBe
                listOf(
                    "session: (job ?x (computer programmer))",
                    """
                    ;;; Amb-Eval input:
                    (job ?x (computer programmer))
                    ;;; Starting a new problem
                    ;;; Amb-Eval value:
                    (job (Hacker Alyssa P) (computer programmer))
                    ;;; Amb-Eval input:
                    try-again
                    ;;; Amb-Eval value:
                    (job (Fect Cy D) (computer programmer))
                    ;;; Amb-Eval input:
                    try-again
                    ;;; There are no more values of
                    the pending problem
                    ;;; Amb-Eval input:
                    try-again
                    ;;; There is no current problem
                    """.trimIndent() + "\n",
                    "session: (or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P))) -- amb is depth-first",
                    """
                    ;;; Amb-Eval input:
                    (or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))
                    ;;; Starting a new problem
                    ;;; Amb-Eval value:
                    (or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))
                    ;;; Amb-Eval input:
                    try-again
                    ;;; Amb-Eval value:
                    (or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))
                    ;;; Amb-Eval input:
                    try-again
                    ;;; Amb-Eval value:
                    (or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))
                    ;;; Amb-Eval input:
                    try-again
                    ;;; Amb-Eval value:
                    (or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))
                    ;;; Amb-Eval input:
                    try-again
                    ;;; There are no more values of
                    the pending problem
                    """.trimIndent() + "\n",
                    "the stream engine interleaves the same disjuncts: 4 answers, first is (or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
                    "session: (married Mickey ?who) through the recursive rule",
                    """
                    ;;; Amb-Eval input:
                    (married Mickey ?who)
                    ;;; Starting a new problem
                    ;;; Amb-Eval value:
                    (married Mickey Minnie)
                    ;;; Amb-Eval input:
                    try-again
                    ;;; Amb-Eval value:
                    (married Mickey Minnie)
                    """.trimIndent() + "\n",
                )
        }
        test("Exercise 4.78: the or restores the or-entry frame, so both disjuncts answer in order") {
            val demos = ambQueryDemos()
            val session = demos[demos.indexOf(OR_SESSION_LABEL) + 1]
            answerValues(session) shouldBe
                listOf(
                    "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
                    "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))",
                    "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))",
                    "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
                )
        }
        test("Exercise 4.78: an or nested under an and restores the and's entry frame: restricted join") {
            val shared = QuerySystem()
            shared.load(microshaftDatabase)
            val amb = ambDriver({ env, random -> QueryAmbEvaluator(env, random, shared) }, "")
            val answers = mutableListOf<String>()
            var round = amb.input("(and (supervisor ?x ?y) (or (job ?x ?j) (salary ?x ?s)))")
            while (!round.contains("There are no more values")) {
                answerValues(round).forEach(answers::add)
                round = amb.input("try-again")
            }
            answers shouldBe
                listOf(
                    "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (or (job (Hacker Alyssa P) (computer programmer)) (salary (Hacker Alyssa P) ?s)))",
                    "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (or (job (Hacker Alyssa P) ?j) (salary (Hacker Alyssa P) 40000)))",
                    "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (or (job (Fect Cy D) (computer programmer)) (salary (Fect Cy D) ?s)))",
                    "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (or (job (Fect Cy D) ?j) (salary (Fect Cy D) 35000)))",
                    "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (or (job (Tweakit Lem E) (computer technician)) (salary (Tweakit Lem E) ?s)))",
                    "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (or (job (Tweakit Lem E) ?j) (salary (Tweakit Lem E) 25000)))",
                    "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (or (job (Reasoner Louis) (computer programmer trainee)) (salary (Reasoner Louis) ?s)))",
                    "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (or (job (Reasoner Louis) ?j) (salary (Reasoner Louis) 30000)))",
                    "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (or (job (Bitdiddle Ben) (computer wizard)) (salary (Bitdiddle Ben) ?s)))",
                    "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (or (job (Bitdiddle Ben) ?j) (salary (Bitdiddle Ben) 60000)))",
                    "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (or (job (Scrooge Eben) (accounting chief accountant)) (salary (Scrooge Eben) ?s)))",
                    "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (or (job (Scrooge Eben) ?j) (salary (Scrooge Eben) 75000)))",
                    "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (or (job (Cratchet Robert) (accounting scrivener)) (salary (Cratchet Robert) ?s)))",
                    "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (or (job (Cratchet Robert) ?j) (salary (Cratchet Robert) 18000)))",
                    "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (or (job (Aull DeWitt) (administration secretary)) (salary (Aull DeWitt) ?s)))",
                    "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (or (job (Aull DeWitt) ?j) (salary (Aull DeWitt) 25000)))",
                )
        }
    })

private const val OR_SESSION_LABEL =
    "session: (or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P))) -- amb is depth-first"

/** The delivered answer of each `;;; Amb-Eval value:` round: the line
 * the driver printed after the marker. */
private fun answerValues(session: String): List<String> {
    val lines = session.lines()
    return lines.withIndex().filter { it.value == ";;; Amb-Eval value:" }.map { lines[it.index + 1] }
}
