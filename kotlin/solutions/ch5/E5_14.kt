// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.14: measuring the factorial machine. The machine
// of figure 5.11 runs for n = 1 to 6 and the monitored stack reports the
// totals; one run carries the book's own measuring trick, a `perform` of
// `print-stack-statistics` inserted before `fact-done`, so the printed
// line and the read counters agree. The relation is linear: two pushes
// per recursive level, so total pushes and maximum depth are both
// 2(n - 1).

package sicp.ch5.solutions

import sicp.ch5.arithOperations
import sicp.ch5.constV
import sicp.ch5.getRegisterContents
import sicp.ch5.labelSrc
import sicp.ch5.makeMachine
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.ch5.setRegisterContents
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.OpAct
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VInt

/** The recursive factorial controller of figure 5.11, transcribed line
 *  for line: each level saves `continue` and `n` for the recursive
 *  call and restores the pair at `after-fact`. */
public val recursiveFactorialSimController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("fact-done")),
        Label("fact-loop"),
        Test(opCond("=", reg("n"), constV(1))),
        Branch("base-case"),
        Save("continue"),
        Save("n"),
        Assign("n", opSrc("-", reg("n"), constV(1))),
        Assign("continue", labelSrc("after-fact")),
        Goto(GotoTarget.Lbl("fact-loop")),
        Label("after-fact"),
        Restore("n"),
        Restore("continue"),
        Assign("val", opSrc("*", reg("n"), reg("val"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("base-case"),
        Assign("val", constV(1)),
        Goto(GotoTarget.ByReg("continue")),
        Label("fact-done"),
    )

/** The controller with the book's measuring `perform` before fact-done. */
private val measuredController: List<Stmt> =
    recursiveFactorialSimController.dropLast(1) +
        Perform(OpAct("print-stack-statistics", emptyList())) +
        recursiveFactorialSimController.takeLast(1)

/** One run of a controller with input n, answering the stack statistics
 *  line the machine's stack reports. */
private fun runStatistics(
    controller: List<Stmt>,
    n: Long,
): String =
    machineRun {
        val machine = makeMachine(listOf("n", "continue", "val"), arithOperations, controller)
        machine.setRegisterContents("n", VInt(n))
        machine.start()
        machine.stack.statistics()
    }

/** The table for n = 1 to 6, then the printed line of the measured
 *  machine for n = 5. */
public fun factorialStackStatistics(): List<String> {
    val table =
        (1L..6L).map { n -> "n = $n: ${runStatistics(recursiveFactorialSimController, n)}" }
    val printed = "the measured machine for n = 5 prints: ${runStatistics(measuredController, 5)}"
    return table + printed
}

/** The factorial value itself, for the oracle pairing in the test. */
public fun factorialMachineFactorial(n: Long): Long =
    machineRun {
        val machine = makeMachine(listOf("n", "continue", "val"), arithOperations, recursiveFactorialSimController)
        machine.setRegisterContents("n", VInt(n))
        machine.start()
        (machine.getRegisterContents("val") as VInt).n
    }
