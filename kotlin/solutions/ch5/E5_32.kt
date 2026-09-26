// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.32: the symbol-operator fast path. (a) The
// modified `ev-application` tests whether the operator is a symbol; if
// it is, the lookup happens in place, without saving `env` or `unev`
// and without the round trip through `eval-dispatch`, and control joins
// the ordinary path with `proc` already set. (b) Alyssa's suggestion is
// the interpreter's trap: the dispatch runs per evaluation, so every
// special case costs a test on every run, and the tests can only see
// the shape of the expression, not the register analysis `preserving`
// does at compile time; the rationale records the argument.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.Op
import sicp.ch5.evaluatorControllerFragments
import sicp.ch5.labelSrc
import sicp.ch5.makeEvaluator
import sicp.ch5.monitoredEvaluatorController
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VPair
import sicp.runtime.VSym

/** The fast-path `ev-application`: the symbol-operator test sits right
 *  after `unev` takes the operands, before `env` and `unev` are
 *  saved. */
public val evApplicationFast: List<Stmt> =
    listOf(
        Label("ev-application"),
        Save("continue"),
        Assign("unev", opSrc("operands", reg("exp"))),
        Test(opCond("symbol-operator?", reg("exp"))),
        Branch("ev-appl-symbol-operator"),
        Save("env"),
        Save("unev"),
        Assign("exp", opSrc("operator", reg("exp"))),
        Assign("continue", labelSrc("ev-appl-did-operator")),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("ev-appl-symbol-operator"),
        Assign("exp", opSrc("operator", reg("exp"))),
        Assign("val", opSrc("lookup-variable-value", reg("exp"), reg("env"))),
        Assign("argl", opSrc("empty-arglist")),
        Assign("proc", reg("val")),
        Test(opCond("no-operands?", reg("unev"))),
        Branch("apply-dispatch"),
        Save("proc"),
        Goto(GotoTarget.Lbl("ev-appl-operand-loop")),
    )

private val fastController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        if (name == "ev-application") evApplicationFast else stmts
    }

/** The one operation the dispatch names: the operator of an
 *  application is a bare symbol. */
public val symbolOperatorOp: Pair<String, Op> =
    "symbol-operator?" to
        { args ->
            if (args.size != 1) raise(sicp.ch5.EvaluatorFault("symbol-operator? needs one argument"))
            val exp = args[0]
            VBool(exp is VPair && exp.car is VSym)
        }

/** Evaluates [source] on the fast-path evaluator and answers the
 *  transcript. */
private fun runFast(source: String): List<String> =
    either {
        val evaluator = makeEvaluator(source, fastController, mapOf(symbolOperatorOp))
        evaluator.drive()
        evaluator.transcript
    }.fold(
        { e -> error("the fast-path run failed: $e") },
        { it },
    )

/** The exercise's runs: symbol calls answer as before, the compound
 *  operator still evaluates through `eval-dispatch`, and the base
 *  monitored factorial of 5 costs the book's 144 pushes, which the
 *  fast path removes the `env` and `unev` saves from. Only the base
 *  run's stack statistics ride along, so the value announcements stay
 *  the fast-path session's own. */
public fun symbolOperatorRuns(): List<String> {
    val session =
        runFast(
            """
            (define (f x) (* x x))
            (f 6)
            ((lambda (y) (+ y 1)) 41)
            """.trimIndent(),
        )
    val basePushes =
        runMonitoredOn(
            monitoredEvaluatorController,
            """
            (define (factorial n)
              (if (= n 1)
                  1
                  (* n (factorial (- n 1)))))
            (factorial 5)
            """.trimIndent(),
        )
    return session + basePushes.filter { it.startsWith("(total-pushes") }
}
