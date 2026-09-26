// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.19: breakpoints. A breakpoint names an instruction
// (by label) and an n: the machine stops just before executing that
// instruction for the n-th time, reports where it is waiting, and resumes
// on `proceed`; `cancel` removes the breakpoint so later runs go straight
// through.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.ch5.Machine
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.arithOperations
import sicp.ch5.getRegisterContents
import sicp.ch5.setRegisterContents
import sicp.runtime.Reg
import sicp.runtime.VInt

/** The breakpoint machine: the execution loop checks the breakpoints
 *  before every instruction and parks the machine before an instruction's
 *  n-th execution, and again before each later n-th. */
public class BreakpointMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    private class Breakpoint(
        val address: Int,
        val every: Int,
    ) {
        var executions = 0
    }

    private val byLabel = LinkedHashMap<String, Breakpoint>()

    /** The label whose breakpoint the machine is waiting at, if any. */
    public var waitingAt: String? = null
        private set

    /** Sets a breakpoint: stop just before the instruction at [label] is
     *  executed for the [n]-th time. */
    public fun setBreakpoint(
        label: String,
        n: Int,
    ) {
        val address = labels[label] ?: throw IllegalArgumentException("no such label: $label")
        byLabel[label] = Breakpoint(address, n)
    }

    /** Removes the breakpoint at [label]. */
    public fun cancelBreakpoint(label: String) {
        byLabel.remove(label)
    }

    /** Removes every breakpoint, the book's cancel-all. */
    public fun cancelAllBreakpoints() {
        byLabel.clear()
    }

    /** Resumes a machine parked at a breakpoint: the stop it is waiting
     *  at is consumed, so the pending execution runs and the machine
     *  stops next at a later arrival. */
    context(r: Raise<MachineError>)
    public fun proceed() {
        val resume = waitingAt
        waitingAt = null
        if (resume != null) {
            byLabel[resume]?.executions += 1
        }
        execute()
    }

    context(r: Raise<MachineError>)
    override fun execute() {
        while (pc < insts.size) {
            val hit = byLabel.entries.firstOrNull { it.value.address == pc }
            val bp = hit?.value
            if (bp != null && bp.executions % bp.every == bp.every - 1) {
                waitingAt = hit.key
                return
            }
            if (bp != null) {
                bp.executions += 1
            }
            insts[pc].exec(r)
        }
    }
}

/** One breakpoint session on the gcd machine: stop before the second
 *  execution of `test-b`, read the registers mid-run, proceed to the
 *  answer, then cancel and restart straight through. */
public fun breakpointSession(): List<String> =
    machineRun {
        val machine = BreakpointMachine(listOf("a", "b", "t"), arithOperations)
        machine.install(gcdController)
        machine.setRegisterContents("a", VInt(206))
        machine.setRegisterContents("b", VInt(40))
        machine.setBreakpoint("test-b", 2)
        machine.start()
        val lines = mutableListOf<String>()
        lines +=
            "break at ${machine.waitingAt}: a = ${machine.getRegisterContents("a")}, " +
            "b = ${machine.getRegisterContents("b")}"
        while (machine.waitingAt != null) {
            machine.proceed()
        }
        lines += "proceed: gcd(206, 40) = ${machine.getRegisterContents("a")}"
        machine.cancelBreakpoint("test-b")
        machine.start()
        lines += "cancel and restart: gcd(206, 40) = ${machine.getRegisterContents("a")}"
        lines
    }
