// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program fact-machine in SICP section 5.1.4
//
// Chapter 5, exercise 5.14: measuring the factorial machine. The machine
// of figure 5.11 runs for n = 1 to 6 and the monitored stack reports the
// totals; one run carries the book's own measuring trick, a `perform` of
// `print-stack-statistics` inserted before `fact-done`, so the printed
// line and the read counters agree. The relation is linear: two pushes
// per recursive level, so total pushes and maximum depth are both
// 2(n - 1).

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.NO_POSITION
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.Perform
import sicp.runtime.Stmt

/** The controller with the book's measuring `perform` after fact-done. */
private val measuredController: List<Stmt> =
    recursiveFactorialController +
        Perform(opAct("print-stack-statistics"))

/** The binding the measuring operation needs: the device reports the
 *  running machine's own stack counters. */
private class StatisticsPrinter {
    var machine: Machine? = null

    fun device(out: StringBuilder): MachineOp =
        { _ ->
            val running = machine ?: raise(GuestError.UnassignedRead(NO_POSITION))
            out.appendLine(statisticsLine(running))
            GValue.VUnit
        }
}

/** One run of [controller] with input n, answering the stack statistics
 *  line the machine's stack reports. */
private fun runStatistics(
    controller: List<Stmt>,
    n: Long,
): String {
    val machine =
        freshMachine(
            setOf("n", "continue", "val"),
            machineArithmetic,
            controller,
            mapOf("n" to GValue.VLong(n)),
        )
    runToHalt(machine)
    return statisticsLine(machine)
}

/** The measured machine's own printed line: the controller's `perform` of
 *  `print-stack-statistics` writes it during the run, and it equals the
 *  counters read back after the halt. */
private fun runMeasuredPrint(n: Long): String {
    val printed = StringBuilder()
    val printer = StatisticsPrinter()
    val ops = machineArithmetic + ("print-stack-statistics" to printer.device(printed))
    val machine =
        freshMachine(
            setOf("n", "continue", "val"),
            ops,
            measuredController,
            mapOf("n" to GValue.VLong(n)),
        )
    printer.machine = machine
    runToHalt(machine)
    return printed.toString().trimEnd()
}

/** The table for n = 1 to 6, then the printed line of the measured
 *  machine for n = 5. */
public fun factorialStackStatistics(): List<String> {
    val table = (1L..6L).map { n -> "n = $n: ${runStatistics(recursiveFactorialController, n)}" }
    val printed = "the measured machine for n = 5 prints: ${runMeasuredPrint(5)}"
    return table + printed
}

/** The factorial value itself, for the oracle pairing in the test. */
public fun factorialMachineFactorial(n: Long): Long {
    val machine =
        freshMachine(
            setOf("n", "continue", "val"),
            machineArithmetic,
            recursiveFactorialController,
            mapOf("n" to GValue.VLong(n)),
        )
    runToHalt(machine)
    return (machine.registers.getValue("val").content as GValue.VLong).value
}
