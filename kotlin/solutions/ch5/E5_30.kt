// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.30: errors signaled inside the evaluator. Both
// parts are built, and the places that needed the change are the two the
// exercise's answer names. (a) `lookup-variable-value` answers a
// distinguished condition code for an unbound variable -- a tagged word
// whose tag no object-language value can carry, because the object
// language has no tagged words -- and `ev-variable` tests the code and
// goes to `signal-error`. (b) `apply-primitive-procedure` checks every
// application: a wrong operand count, `car` of a non-pair, or division by
// zero returns the condition code instead of escaping the machine, and
// `primitive-apply` tests it. Both paths land in the base controller's
// `signal-error`, which stops the machine with the object-level message.

package sicp.ch5.solutions

import arrow.core.Either
import arrow.core.raise.either
import sicp.ch5.Evaluator
import sicp.ch5.EvaluatorFault
import sicp.ch5.Op
import sicp.ch5.applyObjectPrimitive
import sicp.ch5.conditionDetail
import sicp.ch5.conditionKind
import sicp.ch5.conditionWord
import sicp.ch5.evaluatorControllerFragments
import sicp.ch5.isConditionWord
import sicp.ch5.isPrimitiveProcedure
import sicp.ch5.listItems
import sicp.ch5.makeEvaluator
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.primitiveName
import sicp.ch5.reg
import sicp.ch5.symbolNameOf
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Restore
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool

/** The per-machine checking: the machine is absent while it is under
 *  construction; the operations run only once [runChecking] drives it. */
private class Checking {
    var evaluator: Evaluator? = null
}

/** The exercise's operations: the checking lookup and the checking
 *  primitive application, plus the two condition-code tests, over the
 *  evaluator's environment table. */
private fun checkingOperations(checking: Checking): Map<String, Op> =
    mapOf(
        "lookup-variable-value" to
            { args ->
                if (args.size != 2) raise(EvaluatorFault("lookup-variable-value needs two arguments"))
                val name = symbolNameOf(args[0])
                val holder = requireNotNull(checking.evaluator).environmentOf("lookup-variable-value", args[1])
                either { holder.lookup(name) }.fold(
                    { _ -> conditionWord("unbound-variable", "unbound variable: $name") },
                    { it },
                )
            },
        "variable-lookup-failed?" to
            { args ->
                if (args.size != 1) raise(EvaluatorFault("variable-lookup-failed? needs one argument"))
                VBool(isConditionWord(args[0]) && conditionKind(args[0]) == "unbound-variable")
            },
        "apply-primitive-procedure" to
            { args ->
                if (args.size != 2 || !isPrimitiveProcedure(args[0])) {
                    raise(EvaluatorFault("apply-primitive-procedure needs a procedure and an operand list"))
                }
                val values = listItems("apply-primitive-procedure", args[1])
                when (val applied = applyObjectPrimitive(primitiveName(args[0]), values)) {
                    is Either.Left -> conditionWord("primitive-failure", applied.value.detail)
                    is Either.Right -> applied.value
                }
            },
        "primitive-application-failed?" to
            { args ->
                if (args.size != 1) {
                    raise(EvaluatorFault("primitive-application-failed? needs one argument"))
                }
                VBool(isConditionWord(args[0]) && conditionKind(args[0]) == "primitive-failure")
            },
        "signal-error" to
            { args ->
                if (args.size != 1) raise(EvaluatorFault("signal-error needs one argument"))
                raise(EvaluatorFault(conditionDetail(args[0])))
            },
    )

/** The checking `ev-variable`: tests the lookup's condition code before
 *  continuing. */
private val evVariableChecking: List<Stmt> =
    listOf(
        Label("ev-variable"),
        Assign("val", opSrc("lookup-variable-value", reg("exp"), reg("env"))),
        Test(opCond("variable-lookup-failed?", reg("val"))),
        Branch("variable-lookup-failed"),
        Goto(GotoTarget.ByReg("continue")),
        Label("variable-lookup-failed"),
        Goto(GotoTarget.Lbl("signal-error")),
    )

/** The checking `primitive-apply`: tests the application's condition code
 *  before restoring `continue`. */
private val primitiveApplyChecking: List<Stmt> =
    listOf(
        Label("primitive-apply"),
        Assign("val", opSrc("apply-primitive-procedure", reg("proc"), reg("argl"))),
        Test(opCond("primitive-application-failed?", reg("val"))),
        Branch("primitive-application-failed"),
        Restore("continue"),
        Goto(GotoTarget.ByReg("continue")),
        Label("primitive-application-failed"),
        Goto(GotoTarget.Lbl("signal-error")),
    )

/** The exercise's controller: the base controller with the two checking
 *  entries. */
private val checkingController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        when (name) {
            "ev-variable" -> evVariableChecking
            "primitive-apply" -> primitiveApplyChecking
            else -> stmts
        }
    }

/** One run to its stopping point: the stop note first (`end of input`
 *  for the queue-dry end), then the transcript. */
private fun runChecking(source: String): List<String> {
    val checking = Checking()
    val built =
        either { makeEvaluator(source, checkingController, checkingOperations(checking)) }.fold(
            { e -> error("the checking evaluator failed to build: $e") },
            { it },
        )
    checking.evaluator = built
    val note = built.drive() ?: "end of input"
    return listOf(note) + built.transcript
}

/** The caught failures, each pinned by the test, and then a clean
 *  factorial that still answers 120: `car` of a symbol, division by
 *  zero, an unbound variable, a wrong operand count. */
public fun errorSignalingRuns(): List<String> =
    listOf(
        runChecking("(car 5)"),
        runChecking("(/ 1 0)"),
        runChecking("(+ 1 no-such-variable)"),
        runChecking("(remainder 7)"),
        runChecking(
            """
            (define (factorial n)
              (if (= n 1)
                  1
                  (* n (factorial (- n 1)))))
            (factorial 5)
            """.trimIndent(),
        ),
    ).flatten()
