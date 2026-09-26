// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.47: compiled procedures call interpreted procedures.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private const val COMPILED_CALLER = "(define (f x) (g x))"
private const val INTERPRETED_CALLEE_AND_CALL = "(define (g x) (+ x 1)) (f 41)"

/** Runs a compiled caller against a procedure defined later by the evaluator. */
public fun compoundCallRuns(): List<String> =
    valuesOf(
        runCompiled(
            cfg = CompilerConfig(compoundCalls = true),
            compiled = COMPILED_CALLER,
            driver = INTERPRETED_CALLEE_AND_CALL,
        ),
    )
