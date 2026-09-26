// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.48: `compile-and-run` as evaluator primitive.
// The primitive stashes its quoted argument and answers `ok`; between
// phases the solution compiles the stashed form the way `compile-and-go`
// would, and the second machine runs the block and answers the book's
// session: `ok` from the primitive, `ok` from the compiled define,
// then 120.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.EvaluatorFault
import sicp.ch5.Linkage
import sicp.ch5.ObjectPrimitive
import sicp.ch5.compileAndGo
import sicp.ch5.compileBlock
import sicp.ch5.makeCompiledEvaluator
import sicp.runtime.VSym
import sicp.runtime.Value

private val factorialDefineSource: String =
    """
    (compile-and-run
     '(define (factorial n)
        (if (= n 1)
            1
            (* (factorial (- n 1)) n))))
    """.trimIndent()

/** Runs the book's session across two machines and answers both transcripts. */
public fun compileAndRunSession(): List<String> {
    val stashed = ArrayList<Value>()
    val stashAndOk: ObjectPrimitive = { args ->
        if (args.size != 1) raise(EvaluatorFault("compile-and-run needs one argument"))
        stashed.add(args[0])
        VSym("ok")
    }
    val first =
        either {
            val evaluator = makeCompiledEvaluator(factorialDefineSource, extraPrimitives = mapOf("compile-and-run" to stashAndOk))
            evaluator.drive()
            evaluator.transcript
        }.fold({ error("the compile-and-run session failed: $it") }, { it })
    check(stashed.size == 1) { "compile-and-run stashed no definition" }
    val second =
        either {
            val state = CompilerState()
            val (entry, block) = compileBlock(CompilerConfig(), state, listOf(stashed[0]))
            val evaluator = compileAndGo(entry, block, "(factorial 5)")
            evaluator.drive()
            evaluator.transcript
        }.fold({ error("the compiled block run failed: $it") }, { it })
    return listOf(
        "compile-and-run answers: ${valuesOf(first).joinToString(" ")}",
        "compiled block answers: ${valuesOf(second).joinToString(" ")}",
    )
}
