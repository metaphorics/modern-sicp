// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_78

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

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
    })

private const val OR_SESSION_LABEL =
    "session: (or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P))) -- amb is depth-first"

/** The delivered answer of each `;;; Amb-Eval value:` round: the line
 * the driver printed after the marker. */
private fun answerValues(session: String): List<String> {
    val lines = session.lines()
    return lines.withIndex().filter { it.value == ";;; Amb-Eval value:" }.map { lines[it.index + 1] }
}
