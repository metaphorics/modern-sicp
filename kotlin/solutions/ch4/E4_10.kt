// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons

// Exercise 4.10: syntax in the interpreter is not sacred -- redefine it.
// This host answers at the reader level: [syntaxize] is a `Value`->
// `Value` transform that rewrites `(defun name (params...) body...)` into
// the procedure-defining `define` sugar and `(fun (params...) body...)`
// into `lambda`, before the parser ever sees the program. The rewrite is
// structural and recursive: operands, parameter lists, and bodies all
// pass through [syntaxize], so a `fun` nested inside a `defun` body comes
// out as a `lambda`. A malformed `defun`/`fun` passes through untouched
// and the interpreter reports it -- `defun` is then just an unbound head.

/** Rebuilds [v] with syntaxized parts; the malformed-form pass-through. */
private fun passThrough(v: VPair): Value = cons(syntaxize(v.car), syntaxize(v.cdr))

/** `(defun name (params...) body...)` => `(define (name params...) body...)`. */
private fun defunToDefine(v: VPair): Value {
    val rest = v.cdr as? VPair ?: return passThrough(v)
    val name = rest.car as? VSym ?: return passThrough(v)
    val paramsAndBody = rest.cdr as? VPair ?: return passThrough(v)
    val signature = cons(name, syntaxize(paramsAndBody.car))
    return cons(VSym("define"), cons(signature, syntaxize(paramsAndBody.cdr)))
}

/** `(fun (params...) body...)` => `(lambda (params...) body...)`. */
private fun funToLambda(v: VPair): Value {
    val rest = v.cdr as? VPair ?: return passThrough(v)
    return cons(VSym("lambda"), cons(syntaxize(rest.car), syntaxize(rest.cdr)))
}

/** The defun/fun rewrite, applied depth-first over the datum tree. */
public fun syntaxize(v: Value): Value =
    when {
        v is VPair && v.car == VSym("defun") -> defunToDefine(v)
        v is VPair && v.car == VSym("fun") -> funToLambda(v)
        v is VPair -> passThrough(v)
        else -> v
    }

/** The demo program: a `defun` definition, a call, and a higher-order
 * `defun` whose body carries a nested `fun`. */
private val PROGRAM: String =
    """
    (defun cube (x) (* x x x))
    (cube 7)
    (defun twice (f) (fun (x) (f (f x))))
    ((twice cube) 2)
    """.trimIndent()

/** Runs [text], handing the read forms through [prepare] before parsing. */
private fun runDemo(
    text: String,
    prepare: (List<Value>) -> List<Value>,
): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val evaluator = Evaluator(env)
    either {
        val exprs = parseProgram(prepare(readProgram(text)))
        for (expr in exprs) {
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

/** The demo under the transform: the nested `fun` became a `lambda`, so
 * `twice` composes. => "343\n512\n" */
public fun defunTranscript(): String = runDemo(PROGRAM) { forms -> forms.map { syntaxize(it) } }

/** The same program without the transform: `defun` is an unbound head
 * and the run stops there. => "Error: unbound variable: defun\n" */
public fun defunPlainTranscript(): String = runDemo(PROGRAM) { forms -> forms }
