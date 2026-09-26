// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.39: lexical-address lookup and assignment.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.ch5.EvaluatorFault
import sicp.ch5.MachineError
import sicp.runtime.Env
import sicp.runtime.VSym
import sicp.runtime.Value

/** Reads the value at [frameNumber] and [displacement] in the runtime environment. */
context(r: Raise<MachineError>)
public fun lexicalAddressLookup(
    frameNumber: Int,
    displacement: Int,
    env: Env,
): Value {
    val frame = lexicalFrame(frameNumber, env)
    val entry =
        frame.frame.entries.elementAtOrNull(displacement)
            ?: r.raise(EvaluatorFault("lexical address ($frameNumber $displacement) is out of range"))
    if (entry.value == VSym("*unassigned*")) {
        r.raise(EvaluatorFault("variable ${entry.key} is unassigned"))
    }
    return entry.value
}

/** Rebinds the slot at [frameNumber] and [displacement] in the runtime environment. */
context(r: Raise<MachineError>)
public fun lexicalAddressSet(
    frameNumber: Int,
    displacement: Int,
    value: Value,
    env: Env,
) {
    val frame = lexicalFrame(frameNumber, env)
    val name =
        frame.frame.keys.elementAtOrNull(displacement)
            ?: r.raise(EvaluatorFault("lexical address ($frameNumber $displacement) is out of range"))
    frame.define(name, value)
}

context(r: Raise<MachineError>)
private fun lexicalFrame(
    frameNumber: Int,
    env: Env,
): Env {
    if (frameNumber < 0) r.raise(EvaluatorFault("lexical frame $frameNumber is out of range"))
    var frame: Env? = env
    repeat(frameNumber) { frame = frame?.parent }
    return frame ?: r.raise(EvaluatorFault("lexical frame $frameNumber is out of range"))
}

/** Runs lookup and assignment against nested runtime frames. */
public fun lexicalMachineRuns(): List<String> {
    val global = Env.global()
    global.define("g", VSym("global"))
    val outer = Env.child(global)
    outer.define("x", VSym("outer"))
    val inner = Env.child(outer)
    inner.define("y", VSym("inner"))
    val before =
        arrow.core.raise.either<MachineError, Value> {
            lexicalAddressLookup(0, 0, inner)
        }
    val outerValue =
        arrow.core.raise.either<MachineError, Value> {
            lexicalAddressLookup(1, 0, inner)
        }
    val after =
        arrow.core.raise.either<MachineError, Unit> {
            lexicalAddressSet(1, 0, VSym("changed"), inner)
        }
    val changed =
        arrow.core.raise.either<MachineError, Value> {
            lexicalAddressLookup(1, 0, inner)
        }

    fun render(result: arrow.core.Either<MachineError, Value>): String =
        when (result) {
            is arrow.core.Either.Left -> "error: ${result.value}"
            is arrow.core.Either.Right -> result.value.toString()
        }
    return listOf(
        "inner: ${render(before)}",
        "outer before: ${render(outerValue)}",
        "set: ${if (after is arrow.core.Either.Right) "ok" else "error"}",
        "outer after: ${render(changed)}",
    )
}
