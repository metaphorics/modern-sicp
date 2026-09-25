// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.14

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Op
import sicp.runtime.SchemeError
import sicp.runtime.VPrimitive
import sicp.runtime.Value
import sicp.runtime.callPrimitive
import sicp.runtime.listItems
import sicp.runtime.vlist

// Exercise 4.14: Louis Reasoner installs the host `map` as a primitive,
// and his evaluator applies primitives plainly -- [LouisMap] routes every
// [VPrimitive] through [callPrimitive], dropping the base evaluator's
// `map`/`apply` interception. The consequence: [louisMapOp]'s handler can
// only [callPrimitive] its procedure argument, and the argument is a
// compound procedure, a [sicp.runtime.VProc], so the first element dies
// with the typed `NotApplicable` fault. Eva Lu Ator defines `map` in the
// object language instead; her `map` is a [sicp.runtime.VProc], the
// evaluator applies compound procedures normally, and the same call
// answers the mapped list.

/** Louis's map: the host list walk, calling the procedure argument as a
 * primitive call -- the only application a plain primitive can perform. */
private val louisMapOp: Op =
    { args ->
        if (args.size != 2) raise(SchemeError.WrongArity("map", "2", args.size))
        vlist(listItems(args[1]).map { callPrimitive(args[0], listOf(it)) })
    }

/** Louis's evaluator: no special cases, every primitive applies as a
 * primitive, compound procedures apply as the base evaluator applies them. */
public class LouisMap(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun applyProcedure(
        procedure: Value,
        arguments: List<Value>,
    ): EvalStep =
        if (procedure is VPrimitive) {
            EvalStep.Done(callPrimitive(procedure, arguments))
        } else {
            super.applyProcedure(procedure, arguments)
        }
}

/** The call both Louis and Eva run. */
private const val MAP_CALL: String = "(map (lambda (x) (* x x)) '(1 2 3))"

/** Eva's object-language definition of map, ahead of the same call. */
private val EVA_PROGRAM: String =
    """
    (define (map p lst)
      (if (null? lst)
          '()
          (cons (p (car lst)) (map p (cdr lst)))))
    $MAP_CALL
    """.trimIndent()

/** Runs [text] under the printer contract, building the environment with
 * [install] after setup and evaluating on [evaluatorFactory]'s evaluator. */
private fun runOn(
    evaluatorFactory: (Env) -> Evaluator,
    text: String,
    install: (Env) -> Unit = { },
): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    install(env)
    val evaluator = evaluatorFactory(env)
    either {
        for (expr in parseProgram(readProgram(text))) {
            if (expr is DefineE) {
                evaluator.eval(expr, env) // a define prints nothing
                continue
            }
            sink.line(printValue(evaluator.eval(expr, env)))
        }
    }.fold(
        { e -> sink.line("Error: ${sicp.ch4.formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** Louis: the lambda is a compound procedure, and his map primitive can
 * only call primitives. => "Error: not a procedure: #[compound-procedure]\n" */
public fun louisTranscript(): String = runOn(::LouisMap, MAP_CALL) { env -> env.define("map", VPrimitive("map", louisMapOp)) }

/** Eva: map defined in the object language applies the lambda normally.
 * => "(1 4 9)\n" */
public fun evaTranscript(): String = runOn(::Evaluator, EVA_PROGRAM)
