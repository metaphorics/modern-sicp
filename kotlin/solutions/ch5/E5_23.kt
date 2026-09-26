// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.23: derived expressions -- `cond` and `let` enter
// the evaluator through transformer machine operations, the exercise's
// own suggestion. The dispatch grows two tests, one per derived form,
// ahead of the application test; each new entry transforms `exp` and
// re-enters `eval-dispatch`, so the rest of the controller never knows
// the forms existed. `cond->if` builds the book's chain of `if`s: a
// bodyless clause's consequent is its own test, an `else` clause becomes
// its sequence, and a missing `else` ends the chain in the false literal,
// which is why `(cond ((= 1 2)))` answers `#f`. `let->combination` builds
// the lambda over the binding names and applies it to the binding
// initializers in one expression.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch5.EvaluatorFault
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.applicationDispatchTest
import sicp.ch5.evalDispatchTests
import sicp.ch5.evaluatorControllerFragments
import sicp.ch5.listItems
import sicp.ch5.makeEvaluator
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.ch5.unknownExpressionTypeGoto
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

private val derivedDispatchTests: List<Stmt> =
    listOf(
        Test(opCond("cond?", reg("exp"))),
        Branch("ev-cond"),
        Test(opCond("let?", reg("exp"))),
        Branch("ev-let"),
    )

private val derivedEntries: List<Stmt> =
    listOf(
        Label("ev-cond"),
        Assign("exp", opSrc("cond->if", reg("exp"))),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("ev-let"),
        Assign("exp", opSrc("let->combination", reg("exp"))),
        Goto(GotoTarget.Lbl("eval-dispatch")),
    )

/** The exercise's controller: the base fragments with the dispatch
 *  replaced and the transformer entries appended ahead of the errors. */
private val derivedController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        when (name) {
            "eval-dispatch" -> {
                listOf(Label("eval-dispatch")) + evalDispatchTests + derivedDispatchTests +
                    applicationDispatchTest + unknownExpressionTypeGoto
            }

            "errors" -> {
                derivedEntries + stmts
            }

            else -> {
                stmts
            }
        }
    }

private fun isHead(
    w: Value,
    name: String,
): Boolean = w is VPair && w.car is VSym && (w.car as VSym).name == name

/** [sequenceOf]: one expression is itself, several are a `begin`. */
private fun sequenceOf(body: List<Value>): Value = if (body.size == 1) body[0] else cons(VSym("begin"), vlist(body))

/** The book's `cond->if` over the raw clause list. */
private fun Raise<MachineError>.condToIf(clauses: List<Value>): Value {
    if (clauses.isEmpty()) return VBool(false)
    val clause = clauses.first()
    if (isHead(clause, "else")) return sequenceOf(listItems("cond->if", clause).drop(1))
    val parts = listItems("cond->if", clause)
    val rest = condToIf(clauses.drop(1))
    val consequent =
        if (parts.size == 1) parts[0] else sequenceOf(parts.drop(1))
    return cons(VSym("if"), vlist(listOf(parts[0], consequent, rest)))
}

/** The book's `let->combination`: the body as a lambda over the binding
 *  names, applied to the binding initializers. */
private fun Raise<MachineError>.letToCombination(w: Value): Value {
    val parts = listItems("let->combination", w)
    val bindings = listItems("let->combination", parts[1]).map { listItems("let->combination", it) }
    val names = bindings.map { it[0] }
    val inits = bindings.map { it[1] }
    val lambda = cons(VSym("lambda"), cons(vlist(names), vlist(parts.drop(2))))
    return cons(lambda, vlist(inits))
}

private fun oneWord(
    name: String,
    f: Raise<MachineError>.(Value) -> Value,
): Op =
    { args ->
        if (args.size != 1) raise(EvaluatorFault("$name needs one argument"))
        f(args[0])
    }

/** The exercise's operations: the two syntax tests and the two
 *  transformers. */
private val derivedOperations: Map<String, Op> =
    mapOf(
        "cond?" to oneWord("cond?") { w -> VBool(isHead(w, "cond")) },
        "let?" to oneWord("let?") { w -> VBool(isHead(w, "let")) },
        "cond->if" to
            oneWord("cond->if") { w ->
                requireForm(w, "cond")
                condToIf(listItems("cond->if", (w as VPair).cdr))
            },
        "let->combination" to
            oneWord("let->combination") { w ->
                requireForm(w, "let")
                letToCombination(w)
            },
    )

private fun Raise<MachineError>.requireForm(
    w: Value,
    name: String,
) {
    if (!isHead(w, name)) raise(EvaluatorFault("$name->combination needs a $name"))
}

private val classifySource: String =
    """
    (define (classify n)
      (cond ((= n 0) 'zero)
            ((= n 1) 'one)
            (else 'many)))
    (classify 0)
    (classify 1)
    (classify 7)
    (cond ((= 1 2)))
    (let ((a 2) (b 3)) (* a b))
    """.trimIndent()

/** Runs the cond and let sessions through the extended evaluator: a
 *  three-clause classify with an `else`, a bodyless clause, and a `let`
 *  whose body multiplies `a` by `b`. */
public fun derivedExpressionRuns(): List<String> {
    val evaluator =
        either { makeEvaluator(classifySource, derivedController, derivedOperations) }.fold(
            { e -> error("the derived-expression evaluator failed to build: $e") },
            { it },
        )
    evaluator.drive()
    return evaluator.transcript
}
