// SPDX-License-Identifier: GPL-3.0-only
// Running the evaluator as a program (4.1.4): the global environment with
// its primitive bindings, and the driver that evaluates a top-level form
// sequence and records the transcript per spec/scheme-subset/printer.md --
// defines print nothing, `display` and `newline` applications print their
// side effect with no value line, every other expression prints one value
// line, and an `Error:` line stops the program.

package sicp.ch4

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE

/** Builds the book's `setup-environment`: the primitive bindings over an
 * empty frame, plus `true` and `false`. */
public fun setupEnvironment(out: OutputSink): Env {
    val env = Env.global()
    for (entry in defaultPrimitives(out)) {
        env.define(entry.name, entry.toValue())
    }
    env.define("true", VBool(true))
    env.define("false", VBool(false))
    return env
}

/** The `Error:` line text: message without quotes, irritants in the value
 * form, single spaces. */
public fun formatError(e: SchemeError): String =
    when (e) {
        is SchemeError.UserRaised -> {
            if (e.irritants.isEmpty()) {
                e.message
            } else {
                e.irritants.joinToString(separator = " ", prefix = "${e.message} ") { printValue(it) }
            }
        }

        else -> {
            e.toString()
        }
    }

/** The book's driver loop over a fixed program: parses `text`, evaluates
 * each form in `env`, and returns the transcript this run produced.
 * Running on `sink` keeps every book-facing example println-free. */
public fun runProgram(
    text: String,
    env: Env,
    sink: OutputSink,
): String {
    either {
        val forms = readProgram(text)
        val exprs = parseProgram(forms)
        for (expr in exprs) {
            runForm(expr, env, sink)
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** One top-level form under the printer contract. */
context(r: Raise<SchemeError>)
public fun runForm(
    expr: Expr,
    env: Env,
    sink: OutputSink,
) {
    if (expr is DefineE) {
        Evaluator(env).eval(expr, env) // a define prints nothing
        return
    }
    if (expr is AppE && isSinkCall(expr)) {
        Evaluator(env).eval(expr, env) // display/newline: side effect only
        return
    }
    sink.line(printValue(Evaluator(env).eval(expr, env)))
}

/** Whether the application names one of the two print-into-the-sink
 * primitives, whose value line the driver suppresses. */
private fun isSinkCall(expr: AppE): Boolean {
    val operator = expr.operator as? VarE ?: return false
    return operator.name == "display" || operator.name == "newline"
}

/** Convenience for examples and tests: text in, transcript out, on a fresh
 * global environment. */
public fun runProgram(text: String): String {
    val sink = OutputSink()
    return runProgram(text, setupEnvironment(sink), sink)
}

/** Evaluates one datum-shaped expression, the book's `user-print` slot. */
context(r: Raise<SchemeError>)
public fun evalText(
    text: String,
    env: Env,
): Value {
    val form = readProgram(text).singleOrNull() ?: r.raise(SchemeError.Parse("expected one form"))
    return Evaluator(env).eval(parseExpr(form), env)
}

/** The typed boundary for one expression: `Either.Left` carries the fault.
 * Examples, tests, and solution drivers sit on this boundary. */
public fun evalTextEither(
    text: String,
    env: Env,
): Either<SchemeError, Value> = either { evalText(text, env) }

/** Reads and parses the whole text, or the typed parse fault. */
public fun parseTextEither(text: String): Either<SchemeError, List<Expr>> =
    either {
        parseProgram(readProgram(text))
    }
