// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.16: instruction tracing. The tracing machine
// appends each instruction's text to the transcript just before the
// instruction executes, under the control of a trace switch the driver
// turns on and off between runs; untraced runs leave no lines.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.ch5.Machine
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.arithOperations
import sicp.ch5.setRegisterContents
import sicp.runtime.Reg
import sicp.runtime.VInt

/** The tracing machine: when the switch is on, every instruction's text
 *  is appended before it executes. */
public class TracingMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    /** The trace switch. */
    public var traceOn: Boolean = false

    context(r: Raise<MachineError>)
    override fun execute() {
        while (pc < insts.size) {
            if (traceOn) {
                transcript.appendLine(insts[pc].text)
            }
            insts[pc].exec(r)
        }
    }
}

/** One traced gcd run: the transcript is exactly the executed
 *  instructions, from the first `(test (op =) (reg b) (const 0))` to
 *  the final taken `(branch (label gcd-done))`, never the trailing
 *  label. */
public fun tracedGcdTrace(): List<String> =
    machineRun {
        val machine = TracingMachine(listOf("a", "b", "t"), arithOperations)
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
