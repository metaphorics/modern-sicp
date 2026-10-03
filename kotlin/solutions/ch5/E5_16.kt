// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.16: instruction tracing. The tracing machine
// appends each instruction's text to the transcript just before the
// instruction executes, under the control of a trace switch the driver
// turns on and off between runs; untraced runs leave no lines. The text is
// the book's own machine notation, the spelling the assembler summary of
// 5.12 writes.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.Stmt

/** The trace of one machine run: [line] renders an instruction just before
 *  it executes while [tracing] is on, and answers null to omit the line
 *  (a label definition is not an instruction). */
internal fun traceDrive(
    machine: Machine,
    tracing: () -> Boolean,
    line: (Stmt) -> String?,
): List<String> {
    val lines = mutableListOf<String>()
    while (!machine.halted()) {
        val instruction = machine.controller[machine.pc]
        if (tracing()) line(instruction)?.let { lines.add(it) }
        stepOrFail(machine, instruction)
    }
    return lines
}

/** The tracing machine: when the switch is on, every instruction's text
 *  is appended before it executes. */
public class TracingMachine(
    registerNames: Set<String>,
    ops: Map<String, MachineOp>,
    controller: List<Stmt>,
    initial: Map<String, GValue> = emptyMap(),
) {
    /** The trace switch. */
    public var traceOn: Boolean = false

    private val machine = freshMachine(registerNames, ops, controller, initial)

    /** Runs the machine, answering the trace lines the switch allowed. */
    public fun run(): List<String> = traceDrive(machine, { traceOn }, ::traceLine)
}

private fun traceLine(instruction: Stmt): String? = if (instruction is Label) null else instructionText(instruction)

/** One traced gcd run: the transcript is exactly the executed
 *  instructions, from the first `(test (op =) (reg b) (const 0))` to
 *  the final taken `(branch (label gcd-done))`, never the trailing
 *  label. */
public fun tracedGcdTrace(): List<String> =
    TracingMachine(
        setOf("a", "b", "t"),
        machineArithmetic,
        gcdController,
        mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
    ).apply { traceOn = true }.run()
