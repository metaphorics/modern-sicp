// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.25

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.Evaluator
import sicp.ch4.LazyEvaluator
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.lazyTranscriptOn
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env

// Exercise 4.25: `unless` under delayed arguments. In the lazy evaluator
// `unless` is an ordinary procedure whose condition is forced only when the
// body's `if` demands it and whose two arms are thunks, so the
// `unless`-based `factorial` bottoms out at 1 and the product climbs back
// up: `(factorial 5)` answers 120. In an applicative-order language the
// arms are evaluated before `unless` is ever called, which the armed call
// shows directly -- `(/ 1 0)` raises before `42` is even reached -- and
// which dooms the recursion: `(* n (factorial (- n 1)))` is evaluated on
// every entry, so the descent never reaches the guard. The budgeted run
// turns that unbounded descent into a typed fault instead of a hang.

/** The statement's definitions, shared by the lazy and strict runs. */
private val FACTORIAL_PROGRAM: String =
    """
    (define (unless condition usual-value exceptional-value)
      (if condition exceptional-value usual-value))
    (define (factorial n)
      (unless (= n 1)
              (* n (factorial (- n 1)))
              1))
    (factorial 5)
    """.trimIndent()

/** The recursion under delayed arguments bottoms out and answers.
 * => "120\n" */
public fun lazyFactorialTranscript(): String = lazyTranscriptOn(::LazyEvaluator, FACTORIAL_PROGRAM)

/** The armed call on the strict evaluator: the arms evaluate before
 * `unless` is called, so the exceptional arm raises first.
 * => "Error: division by zero\n" */
public fun strictArmedUnlessTranscript(): String =
    transcriptOn(
        ::Evaluator,
        """
        (define (unless condition usual-value exceptional-value)
          (if condition exceptional-value usual-value))
        (unless (= 1 1) (/ 1 0) 42)
        """.trimIndent(),
    )

/** The same recursion on the strict evaluator under a step budget: the
 * arms evaluate on every entry, so the descent never reaches the guard and
 * the budget fires. => "Error: machine fault: step budget exhausted after
 * 200 steps\n" */
public fun strictFactorialTranscript(): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val evaluator = Bounded(env, 200)
    either {
        for (expr in parseProgram(readProgram(FACTORIAL_PROGRAM))) {
            if (expr is DefineE) {
                evaluator.eval(expr, env) // a define prints nothing
                continue
            }
            sink.line(printValue(evaluator.eval(expr, env)))
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}
