// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.19: breakpoints. A breakpoint names a place in
// the controller and an n: the driver parks just before the instruction
// at that place runs for the n-th time, reports where it is waiting, and
// resumes on `proceed`; `cancel` releases the breakpoint so later runs go
// straight through. The substrate machine has no breakpoint registry, so
// the feature rides the stepping seam the exercise establishes.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Machine

/** The breakpoint driver: it parks the machine before an instruction's
 *  n-th execution and again before each later n-th. */
public class BreakpointDriver(
    private val machine: Machine,
) {
    private class Breakpoint(
        val address: Int,
        val every: Int,
    ) {
        var executions = 0
        var stopped = false
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
        require(n >= 1) { "a breakpoint needs a positive arrival count, got $n" }
        val address = machine.labels[label] ?: error("no such label: $label")
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

    /** Runs until the machine halts or parks at a breakpoint. A fresh run
     *  begins a fresh stop schedule: no stop carries over. */
    public fun start() {
        waitingAt = null
        byLabel.values.forEach { it.stopped = false }
        drive()
    }

    /** Resumes a machine parked at a breakpoint: the pending execution
     *  runs and the machine stops next at a later n-th arrival, not at
     *  the same one forever. */
    public fun proceed() {
        waitingAt = null
        drive()
    }

    private fun drive() {
        while (!machine.halted()) {
            val hit = byLabel.entries.firstOrNull { it.value.address == machine.pc }
            val breakpoint = hit?.value
            val parked = breakpoint != null && !breakpoint.stopped && breakpoint.executions % breakpoint.every == breakpoint.every - 1
            if (parked) {
                waitingAt = hit.key
                breakpoint.stopped = true
                return
            }
            if (breakpoint != null) {
                breakpoint.stopped = false
                breakpoint.executions += 1
            }
            stepOrFail(machine, machine.controller[machine.pc])
        }
    }
}

/** One breakpoint session on the gcd machine: stop before the second
 *  and fourth execution of `test-b`, reading the registers at each
 *  stop, proceed to the answer, then cancel and restart straight
 *  through. */
public fun breakpointSession(): List<String> {
    val lines = mutableListOf<String>()
    val stopped =
        freshMachine(
            setOf("a", "b", "t"),
            machineArithmetic,
            gcdController,
            mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
        )
    val driver = BreakpointDriver(stopped)
    driver.setBreakpoint("test-b", 2)
    driver.start()
    while (driver.waitingAt != null) {
        lines.add(
            "break at ${driver.waitingAt}: a = ${render(stopped.registers.getValue("a").content)}, " +
                "b = ${render(stopped.registers.getValue("b").content)}",
        )
        driver.proceed()
    }
    lines.add("finished: gcd(206, 40) = ${render(stopped.registers.getValue("a").content)}")
    driver.cancelBreakpoint("test-b")
    val restarted =
        freshMachine(
            setOf("a", "b", "t"),
            machineArithmetic,
            gcdController,
            mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
        )
    BreakpointDriver(restarted).start()
    lines.add("cancel and restart: gcd(206, 40) = ${render(restarted.registers.getValue("a").content)}")
    return lines
}
