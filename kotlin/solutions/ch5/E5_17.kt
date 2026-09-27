// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.17: traced lines with their labels. The tracing
// machine keeps the label currently in effect -- the most recently passed
// label definition -- and prints it ahead of each traced instruction, so
// the trace reads like the controller listing.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.ch5.Machine
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.arithOperations
import sicp.ch5.setRegisterContents
import sicp.runtime.Reg
import sicp.runtime.VInt

/** The label-aware tracing machine: a label at the current address is
 *  printed before the instruction text. */
public class LabelTracingMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    /** The trace switch. */
    public var traceOn: Boolean = false

    private var labelAt: Map<Int, String> = emptyMap()
    private var currentLabel: String? = null

    context(r: Raise<MachineError>)
    override fun execute() {
        labelAt = labels.entries.associate { (name, address) -> address to name }
        currentLabel = null
        while (pc < insts.size) {
            labelAt[pc]?.let { currentLabel = it }
            if (traceOn) {
                val label = currentLabel
                transcript.appendLine(
                    if (label == null) insts[pc].text else "$label: ${insts[pc].text}",
                )
            }
            insts[pc].exec(r)
        }
    }
}

/** The traced gcd run with labels: every executed line is named by the
 *  label in effect, `test-b` for the loop, `gcd-done` for nothing (the
 *  machine halts on reaching it). */
public fun labelTracedGcdTrace(): List<String> =
    machineRun {
        val machine = LabelTracingMachine(listOf("a", "b", "t"), arithOperations)
        machine.install(gcdController)
        machine.setRegisterContents("a", VInt(206))
        machine.setRegisterContents("b", VInt(40))
        machine.traceOn = true
        machine.start()
        machine.transcript
            .toString()
            .trimEnd()
            .lines()
    }
