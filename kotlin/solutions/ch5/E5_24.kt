// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.24: `cond` as a basic controller form -- not
// reduced to `if`. The controller gains `ev-cond`, a loop over the clause
// list: each iteration takes the first clause, advances the walk in
// `unev`, and tests the clause's predicate through `eval-dispatch`; a
// true predicate (or an `else`) selects the clause and its actions go to
// `ev-sequence`, so the clause's last expression is still in tail
// position. The clause under test rides in `proc`, whose live value every
// enclosing argument loop has already saved on the stack. Two answers the
// derived form gives for free are handled explicitly: a selected clause
// with no actions returns the predicate's value, and no true clause with
// no `else` answers `#f`, exactly what `cond->if` would have produced.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch5.EvaluatorFault
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.applicationDispatchTest
import sicp.ch5.constBool
import sicp.ch5.evalDispatchTests
import sicp.ch5.evaluatorControllerFragments
import sicp.ch5.evaluatorFragment
import sicp.ch5.labelSrc
import sicp.ch5.makeEvaluator
import sicp.ch5.monitoredDriver
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.ch5.unknownExpressionTypeGoto
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value

private val condDispatchTests: List<Stmt> =
    listOf(
        Test(opCond("cond?", reg("exp"))),
        Branch("ev-cond"),
    )

/** The 5.24 clause loop, the book's exercise answer line for line. */
private val evCond: List<Stmt> =
    listOf(
        Label("ev-cond"),
        Save("continue"),
        Assign("unev", opSrc("cond-clauses", reg("exp"))),
        Label("ev-cond-loop"),
        Test(opCond("no-clauses?", reg("unev"))),
        Branch("ev-cond-no-true-clause"),
        Assign("val", opSrc("first-clause", reg("unev"))),
        Assign("unev", opSrc("rest-clauses", reg("unev"))),
        Test(opCond("cond-else-clause?", reg("val"))),
        Branch("ev-cond-else"),
        Assign("exp", opSrc("cond-predicate", reg("val"))),
        Save("val"),
        Save("unev"),
        Save("env"),
        Save("continue"),
        Assign("continue", labelSrc("ev-cond-decide")),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("ev-cond-decide"),
        Restore("continue"),
        Restore("env"),
        Restore("unev"),
        Restore("proc"),
        Test(opCond("true?", reg("val"))),
        Branch("ev-cond-selected"),
        Goto(GotoTarget.Lbl("ev-cond-loop")),
        Label("ev-cond-selected"),
        Assign("unev", opSrc("cond-actions", reg("proc"))),
        Goto(GotoTarget.Lbl("ev-cond-actions")),
        Label("ev-cond-else"),
        Assign("unev", opSrc("cond-actions", reg("val"))),
        Goto(GotoTarget.Lbl("ev-cond-actions")),
        Label("ev-cond-actions"),
        Test(opCond("no-more-exps?", reg("unev"))),
        Branch("ev-cond-empty-actions"),
        Goto(GotoTarget.Lbl("ev-sequence")),
        Label("ev-cond-empty-actions"),
        Restore("continue"),
        Goto(GotoTarget.ByReg("continue")),
        Label("ev-cond-no-true-clause"),
        Restore("continue"),
        Assign("val", constBool(false)),
        Goto(GotoTarget.ByReg("continue")),
    )

/** The exercise's controller: the base fragments with the dispatch
 *  replaced and the clause loop appended ahead of the errors, on the
 *  [driver] the caller chooses. */
private fun condControllerWith(driver: List<Stmt>): List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        when (name) {
            "driver" -> {
                driver
            }

            "eval-dispatch" -> {
                listOf(Label("eval-dispatch")) + evalDispatchTests + condDispatchTests +
                    applicationDispatchTest + unknownExpressionTypeGoto
            }

            "errors" -> {
                evCond + stmts
            }

            else -> {
                stmts
            }
        }
    }

private val condController: List<Stmt> = condControllerWith(evaluatorFragment("driver"))

private fun isHead(
    w: Value,
    name: String,
): Boolean = w is VPair && w.car is VSym && (w.car as VSym).name == name

private fun carOf(w: Value): Value = (w as VPair).car

private fun cdrOf(w: Value): Value = (w as VPair).cdr

private fun oneWord(
    name: String,
    f: Raise<MachineError>.(Value) -> Value,
): Op =
    { args ->
        if (args.size != 1) raise(EvaluatorFault("$name needs one argument"))
        f(args[0])
    }

/** The exercise's operations: the clause-list selectors of `ev-cond`. */
private val condOperations: Map<String, Op> =
    mapOf(
        "cond?" to oneWord("cond?") { w -> VBool(isHead(w, "cond")) },
        "cond-clauses" to oneWord("cond-clauses") { w -> cdrOf(w) },
        "no-clauses?" to oneWord("no-clauses?") { w -> VBool(w is VNil) },
        "first-clause" to oneWord("first-clause") { w -> carOf(w) },
        "rest-clauses" to oneWord("rest-clauses") { w -> cdrOf(w) },
        "cond-else-clause?" to oneWord("cond-else-clause?") { w -> VBool(isHead(w, "else")) },
        "cond-predicate" to oneWord("cond-predicate") { w -> carOf(w) },
        "cond-actions" to oneWord("cond-actions") { w -> cdrOf(w) },
    )

private val classifySource: String =
    """
    (define (classify n)
      (cond ((= n 0) 'zero)
            ((= n 1) 'one)
            (else 'many)))
    (classify 0)
    (classify 1)
    (classify 7)
    (cond ((= 1 1)))
    (cond ((= 1 2)) ((= 2 3)))
    """.trimIndent()

/** Runs the same observable sessions the derived form answered, through
 *  the clause loop instead of the transformer: the classify, a bodyless
 *  clause whose test is true, and a cond with no true clause and no
 *  `else`. */
public fun condBasicFormRuns(): List<String> {
    val evaluator =
        either { makeEvaluator(classifySource, condController, condOperations) }.fold(
            { e -> error("the cond evaluator failed to build: $e") },
            { it },
        )
    evaluator.drive()
    return evaluator.transcript
}

private val condLoopSource: String =
    """
    (define (loop n)
      (cond ((= n 0) 'done)
            (else (loop (- n 1)))))
    """.trimIndent()

/** The monitored maximum depth of the `(loop n)` call for each [n]: the
 *  tail-position proof. The selected clause's actions run through
 *  `ev-sequence`, so the recursive call is the sequence's last
 *  expression and the depth must not grow with n. */
public fun condTailPositionDepths(ns: List<Int>): List<Int> =
    ns.map { n ->
        statsOf(
            runMonitoredOn(
                condControllerWith(monitoredDriver),
                "$condLoopSource\n(loop $n)",
                condOperations,
            ),
        ).last().depth
    }
