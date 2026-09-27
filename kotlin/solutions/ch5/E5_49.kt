// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.49: a read-compile-execute-print loop. The host
// compiles every form into a block under the shared compile state,
// then assembles one machine whose driver chain prompts, runs each
// block, and prints each value, so definitions persist in the one
// global environment. No interpreter sits anywhere in the path.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.compileBlock
import sicp.ch5.compiledApplyDispatch
import sicp.ch5.evaluatorControllerFragments
import sicp.ch5.labelSrc
import sicp.ch5.makeCompiledEvaluator
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.runtime.Assign
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.OpAct
import sicp.runtime.Perform
import sicp.runtime.Stmt

private val loopForms: List<String> =
    listOf(
        "(define (square n) (* n n))",
        "(square 12)",
        "(define (twice n) (+ n n))",
        "(twice (square 21))",
    )

/** The chain driver: prompt, run the block, and print the value, per entry. */
private fun chainDriver(entries: List<String>): List<Stmt> {
    val stmts = ArrayList<Stmt>()
    stmts.add(Label("read-eval-print-loop"))
    stmts.add(Perform(OpAct("initialize-stack", emptyList())))
    stmts.add(Assign("env", opSrc("get-global-environment")))
    entries.forEachIndexed { index, entry ->
        stmts.add(Perform(OpAct("prompt-for-input", emptyList())))
        stmts.add(Assign("continue", labelSrc("print-$index")))
        stmts.add(Goto(GotoTarget.Lbl(entry)))
        stmts.add(Label("print-$index"))
        stmts.add(Perform(OpAct("announce-output", emptyList())))
        stmts.add(Perform(OpAct("user-print", listOf(reg("val")))))
    }
    stmts.add(Goto(GotoTarget.Lbl("machine-end")))
    return stmts
}

/** Runs the loop over the four forms and answers one joined line per form. */
public fun readCompileExecutePrint(): List<String> {
    val transcript =
        either {
            val state = CompilerState()
            val blocks =
                loopForms.map { form ->
                    compileBlock(CompilerConfig(), state, readForms(form)).second
                }
            val entries = blocks.mapIndexed { index, _ -> "compiled-entry-${index + 1}" }
            val fragments =
                evaluatorControllerFragments.flatMap { (name, stmts) ->
                    when (name) {
                        "driver" -> chainDriver(entries)
                        "apply-dispatch" -> compiledApplyDispatch
                        else -> stmts
                    }
                }
            // The flag stays down, so the external entry is unreachable and
            // stays out: nothing in the chain sets `continue` to its label.
            val controller = fragments + blocks.flatten() + Label("machine-end")
            val evaluator = makeCompiledEvaluator("", controller)
            evaluator.drive()
            evaluator.transcript
        }.fold({ error("the read-compile-execute-print loop failed: $it") }, { it })
    check(transcript.size == 3 * loopForms.size) { "the session printed an unexpected line count: $transcript" }
    return transcript.chunked(3) { group -> group.joinToString(" ") }
}
