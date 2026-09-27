// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.persistentMapOf
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.SchemeError
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.setCdr
import sicp.runtime.vlist

// Exercise 4.12: the book asks for abstractions that capture the pattern
// beneath `lookup-variable-value` and `set-variable-value!`: one per-frame
// search and one frame-chain walk, with the environment operations
// redefined on top. [frameScan] is the `assoc` over one frame's alist
// alist representation (Exercise 4.11's), [envScan] walks the chain to
// the first frame whose [frameScan] hits, and [ScannedFrames] defines
// lookup and `set!` purely on the two scans. `define` conses without
// scanning, matching the book's define-variable!, and extend-environment
// keeps the runtime's arity contract.

/** The frame slot key under which each [Env] carries its alist frame. */
private const val FRAME_KEY: String = "*frame-alist*"

/** The book's per-frame search: the first `(name . value)` pair of
 * [env]'s alist frame, or null when this frame has no binding. */
public fun frameScan(
    name: String,
    env: Env,
): VPair? {
    var cursor: Value = env.frame[FRAME_KEY] ?: VNil
    while (cursor is VPair) {
        val binding = cursor.car
        if (binding is VPair && binding.car == VSym(name)) return binding
        cursor = cursor.cdr
    }
    return null
}

/** The book's frame-chain walk: the nearest `(name . value)` pair on the
 * chain, or null when no frame holds `name`. */
public fun envScan(
    name: String,
    env: Env,
): VPair? {
    var cursor: Env? = env
    while (cursor != null) {
        val binding = frameScan(name, cursor)
        if (binding != null) return binding
        cursor = cursor.parent
    }
    return null
}

/** The setupEnvironment bindings, moved into the global alist frame so
 * the whole chain speaks the alist representation. */
private fun scannedGlobal(sink: OutputSink): Env {
    val env = setupEnvironment(sink)
    var alist: Value = VNil
    for ((name, v) in env.frame) {
        alist = cons(cons(VSym(name), v), alist)
    }
    env.frame = persistentMapOf(FRAME_KEY to alist)
    return env
}

/** The evaluator whose lookup and set! are [frameScan]/[envScan] clients. */
public class ScannedFrames(
    sink: OutputSink,
) : Evaluator(scannedGlobal(sink)) {
    /** The nearest binding's value, found by [envScan]. */
    context(r: Raise<SchemeError>)
    override fun lookupVariable(
        name: String,
        env: Env,
    ): Value = envScan(name, env)?.cdr ?: r.raise(SchemeError.Unbound(name))

    /** Rewrites the nearest binding's value slot, found by [envScan]. */
    context(r: Raise<SchemeError>)
    override fun setVariable(
        name: String,
        value: Value,
        env: Env,
    ) {
        val binding = envScan(name, env) ?: r.raise(SchemeError.Unbound(name))
        binding.setCdr(value)
    }

    /** `define` conses a fresh pair onto the frame's alist front. */
    override fun defineVariable(
        name: String,
        value: Value,
        env: Env,
    ) {
        val alist = env.frame[FRAME_KEY] ?: VNil
        env.frame = env.frame.putting(FRAME_KEY, cons(cons(VSym(name), value), alist))
    }

    /** A fresh alist frame over [parent]; the runtime's arity contract. */
    context(r: Raise<SchemeError>)
    override fun extendEnvironment(
        names: List<String>,
        values: List<Value>,
        parent: Env,
        rest: String?,
    ): Env {
        if (rest == null && names.size != values.size) {
            r.raise(SchemeError.WrongArity("extend", names.size.toString(), values.size))
        }
        if (rest != null && values.size < names.size) {
            r.raise(SchemeError.WrongArity("extend", "at least ${names.size}", values.size))
        }
        val frame = Env.child(parent)
        var alist: Value = VNil
        for (i in names.indices.reversed()) {
            alist = cons(cons(VSym(names[i]), values[i]), alist)
        }
        if (rest != null) {
            alist = cons(cons(VSym(rest), vlist(values.drop(names.size))), alist)
        }
        frame.frame = persistentMapOf(FRAME_KEY to alist)
        return frame
    }
}

/** The define/lookup/set! demo: `bump`'s own frame has no `z`, so the
 * `set!` walks to the global alist frame and rewrites its pair. */
private val DEFINE_PROGRAM: String =
    """
    (define z 2)
    z
    (define (bump) (set! z 10) z)
    (bump)
    z
    """.trimIndent()

/** A binding that lives only in a call frame dies with the call. */
private val FRESH_PROGRAM: String =
    """
    (define (stash) (define z 2) z)
    (stash)
    z
    """.trimIndent()

/** The arity contract of extend-environment, re-checked on the scans. */
private const val ARITY_PROGRAM: String = "((lambda (x y) x) 1)"

/** Runs [text] on a [ScannedFrames] evaluator under the printer contract. */
private fun runScanned(text: String): String {
    val sink = OutputSink()
    val evaluator = ScannedFrames(sink)
    val env = evaluator.global
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

/** The define/lookup/set! demo through the scan abstractions.
 * => "2\n10\n10\n" */
public fun scannedTranscript(): String = runScanned(DEFINE_PROGRAM)

/** `z` died with its call frame; the walk stops without a hit.
 * => "2\nError: unbound variable: z\n" */
public fun scannedFreshFrameTranscript(): String = runScanned(FRESH_PROGRAM)

/** extend-environment still refuses an arity mismatch.
 * => "Error: extend: wrong number of arguments, expected 2, got 1\n" */
public fun scannedArityTranscript(): String = runScanned(ARITY_PROGRAM)
